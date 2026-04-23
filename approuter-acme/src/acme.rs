//! ACME client wrapper. Uses instant-acme 0.8 for the protocol and our own
//! Cloudflare DNS provider for the DNS-01 challenge.

use crate::cf_dns::CfDns;
use anyhow::{anyhow, Context, Result};
use instant_acme::{
    Account, AccountCredentials, AuthorizationStatus, ChallengeType, Identifier, LetsEncrypt,
    NewAccount, NewOrder, OrderStatus, RetryPolicy,
};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tracing::{info, warn};

const DNS_PROPAGATION_WAIT: Duration = Duration::from_secs(25);

pub struct AcmeClient {
    domain: String,
    contact: String,
    cf: CfDns,
    cert_dir: PathBuf,
    zone_name: String,
    staging: bool,
}

pub struct CertBundle {
    pub cert_pem: String,
    pub key_pem: String,
}

impl AcmeClient {
    pub fn new(
        domain: String,
        contact: String,
        cf: CfDns,
        cert_dir: PathBuf,
        zone_name: String,
        staging: bool,
    ) -> Self {
        Self {
            domain,
            contact,
            cf,
            cert_dir,
            zone_name,
            staging,
        }
    }

    /// Load existing cert from disk, if any. Does not validate expiry.
    pub async fn load_existing(&self) -> Result<Option<CertBundle>> {
        let cert_path = self.cert_dir.join(format!("{}.crt", self.domain));
        let key_path = self.cert_dir.join(format!("{}.key", self.domain));
        if !cert_path.exists() || !key_path.exists() {
            return Ok(None);
        }
        let cert_pem = fs::read_to_string(&cert_path).await?;
        let key_pem = fs::read_to_string(&key_path).await?;
        Ok(Some(CertBundle { cert_pem, key_pem }))
    }

    /// Issue a new cert via ACME DNS-01 and persist it to disk.
    pub async fn issue(&self) -> Result<CertBundle> {
        fs::create_dir_all(&self.cert_dir).await?;

        let server = if self.staging {
            LetsEncrypt::Staging.url().to_owned()
        } else {
            LetsEncrypt::Production.url().to_owned()
        };

        // Load or create ACME account.
        let acct_path = self.cert_dir.join("acme_account.json");
        let account = if acct_path.exists() {
            info!("reusing ACME account from {:?}", acct_path);
            let creds_json = fs::read_to_string(&acct_path).await?;
            let creds: AccountCredentials = serde_json::from_str(&creds_json)?;
            Account::builder()?.from_credentials(creds).await?
        } else {
            info!("creating new ACME account at {}", server);
            let (account, creds) = Account::builder()?
                .create(
                    &NewAccount {
                        contact: &[&format!("mailto:{}", self.contact)],
                        terms_of_service_agreed: true,
                        only_return_existing: false,
                    },
                    server,
                    None,
                )
                .await?;
            fs::write(&acct_path, serde_json::to_string_pretty(&creds)?).await?;
            account
        };

        // Create order for our domain.
        info!("creating ACME order for {}", self.domain);
        let identifiers = vec![Identifier::Dns(self.domain.clone())];
        let mut order = account
            .new_order(&NewOrder::new(identifiers.as_slice()))
            .await?;

        // Look up CF zone once.
        let zone_id = self
            .cf
            .zone_id(&self.zone_name)
            .await
            .with_context(|| format!("looking up CF zone id for {}", self.zone_name))?;
        let mut txt_record_ids: Vec<String> = Vec::new();

        // For each authz: publish TXT, wait for propagation, then signal
        // LE to validate. Challenge is a borrow of authz — can't move it out.
        // So we publish + sleep + set_ready together within the loop.
        let mut authorizations = order.authorizations();
        while let Some(result) = authorizations.next().await {
            let mut authz = result?;
            match authz.status {
                AuthorizationStatus::Pending => {}
                AuthorizationStatus::Valid => continue,
                status => return Err(anyhow!("unexpected authz status: {:?}", status)),
            }

            let mut challenge = authz
                .challenge(ChallengeType::Dns01)
                .ok_or_else(|| anyhow!("no DNS-01 challenge offered"))?;

            let identifier = challenge.identifier().to_string();
            let txt_name = format!("_acme-challenge.{}", identifier);
            let txt_value = challenge.key_authorization().dns_value();

            info!("publishing TXT {} = {}", txt_name, txt_value);
            let record_id = self
                .cf
                .add_txt(&zone_id, &txt_name, &txt_value)
                .await
                .context("publishing DNS-01 TXT record")?;
            txt_record_ids.push(record_id);

            info!(
                "waiting {:?} for DNS propagation before set_ready",
                DNS_PROPAGATION_WAIT
            );
            tokio::time::sleep(DNS_PROPAGATION_WAIT).await;

            info!("signaling challenge ready to LE for {}", identifier);
            challenge.set_ready().await?;
        }

        // Poll until order is ready (LE validated) or fails.
        let status = order.poll_ready(&RetryPolicy::default()).await?;
        if status != OrderStatus::Ready {
            return Err(anyhow!("unexpected order status after poll_ready: {:?}", status));
        }

        // Finalize the order — instant-acme generates the key and CSR.
        info!("finalizing order and fetching certificate");
        let key_pem = order.finalize().await?;
        let cert_pem = order.poll_certificate(&RetryPolicy::default()).await?;

        // Clean up challenge TXTs.
        for id in &txt_record_ids {
            if let Err(e) = self.cf.delete_record(&zone_id, id).await {
                warn!("failed to clean up TXT record {}: {}", id, e);
            }
        }

        // Persist to disk.
        let cert_path = self.cert_dir.join(format!("{}.crt", self.domain));
        let key_path = self.cert_dir.join(format!("{}.key", self.domain));
        fs::write(&cert_path, &cert_pem).await?;
        fs::write(&key_path, &key_pem).await?;
        info!("cert written to {:?}", cert_path);

        Ok(CertBundle { cert_pem, key_pem })
    }
}
