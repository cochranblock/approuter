// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Waiver confirmation email. Gmail API (Workspace) or Resend. P20: blocking, no spawn.

/// f78 = send_waiver_confirmation. Why: Blocking per P20. Gmail API for Workspace; Resend fallback.
pub fn f78(to: &str, full_name: &str, ref_id: &str) {
    let creds = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").ok();
    let impersonate = std::env::var("GMAIL_IMPERSONATE_USER").ok();
    if let (Some(path), Some(impersonate_user)) = (
        creds.as_ref().filter(|p| !p.is_empty()),
        impersonate.as_ref().filter(|u| !u.is_empty()),
    ) {
        if std::path::Path::new(path).exists() {
            let from = std::env::var("GMAIL_FROM")
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| impersonate_user.clone());
            if let Err(e) = f78_gmail(path, impersonate_user, &from, to, full_name, ref_id) {
                tracing::warn!("Gmail API failed: {}. Falling back to Resend.", e);
                f78_resend(to, full_name, ref_id);
            }
            return;
        }
    }
    f78_resend(to, full_name, ref_id);
}

/// f78_gmail = send via Gmail API (service account + domain-wide delegation).
/// impersonate_user = JWT sub (real Workspace user). from = From header (can be send-as alias).
fn f78_gmail(
    creds_path: &str,
    impersonate_user: &str,
    from: &str,
    to: &str,
    full_name: &str,
    ref_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subject = "Your waiver is confirmed — OakilyDokily";
    let content = format!(
        r#"<p>Hi {},</p><p>Thank you for signing the OakilyDokily service waiver. We've received and recorded your signature.</p><p><strong>Reference ID:</strong> <code>{}</code></p><p>We keep a copy on file for 7 years. If you ever need a copy, just reach out.</p><p>We look forward to caring for your pets.</p><p>— Kaylie &amp; the OakilyDokily team</p>"#,
        f79(full_name),
        f79(ref_id)
    );
    if from == impersonate_user {
        let client = rust_gmail::GmailClient::builder(creds_path, from)?.build_blocking()?;
        client.send_email_blocking(to, subject, &content)?;
    } else if let Err(e) =
        f78_gmail_custom_from(creds_path, impersonate_user, from, to, subject, &content)
    {
        tracing::warn!(
            "Gmail send as {} failed: {}. Retrying as {}.",
            from,
            e,
            impersonate_user
        );
        let client =
            rust_gmail::GmailClient::builder(creds_path, impersonate_user)?.build_blocking()?;
        client.send_email_blocking(to, subject, &content).map_err(
            |retry_e| -> Box<dyn std::error::Error + Send + Sync> {
                format!(
                    "send-as {} failed: {}; retry as {} failed: {}",
                    from, e, impersonate_user, retry_e
                )
                .into()
            },
        )?;
    }
    tracing::info!("Waiver confirmation email sent via Gmail to {}", to);
    Ok(())
}

/// f78_gmail_custom_from = send with From != impersonate (e.g. noreply@ send-as alias).
fn f78_gmail_custom_from(
    creds_path: &str,
    impersonate_user: &str,
    from: &str,
    to: &str,
    subject: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sa: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(creds_path)?)?;
    let client_email = sa["client_email"].as_str().ok_or("missing client_email")?;
    let private_key = sa["private_key"].as_str().ok_or("missing private_key")?;
    let encoding_key = jsonwebtoken::EncodingKey::from_rsa_pem(private_key.as_bytes())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let claims = serde_json::json!({
        "iss": client_email,
        "sub": impersonate_user,
        "scope": "https://www.googleapis.com/auth/gmail.send",
        "aud": "https://oauth2.googleapis.com/token",
        "iat": now,
        "exp": now + 3600,
    });
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
    let jwt = jsonwebtoken::encode(&header, &claims, &encoding_key)?;
    let token_url = std::env::var("OD_OAUTH2_TOKEN_URL")
        .unwrap_or_else(|_| "https://oauth2.googleapis.com/token".into());
    let token_res = reqwest::blocking::Client::new()
        .post(&token_url)
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .timeout(std::time::Duration::from_secs(10))
        .send()?;
    if !token_res.status().is_success() {
        let status = token_res.status();
        let err = token_res.text().unwrap_or_else(|_| "unknown".into());
        return Err(format!("OAuth token {}: {}", status, err).into());
    }
    let token_json: serde_json::Value = token_res.json()?;
    if let Some(e) = token_json.get("error_description") {
        if let Some(s) = e.as_str() {
            return Err(format!("OAuth: {}", s).into());
        }
    }
    let access_token = token_json["access_token"]
        .as_str()
        .ok_or("missing access_token in token response")?;
    let mime = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}\r\n",
        from, to, subject, content
    );
    let raw = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE, mime.as_bytes());
    let body = serde_json::json!({ "raw": raw });
    let gmail_base = std::env::var("OD_GMAIL_API_BASE")
        .unwrap_or_else(|_| "https://gmail.googleapis.com".into());
    let send_url = format!(
        "{}/gmail/v1/users/me/messages/send",
        gmail_base.trim_end_matches('/')
    );
    let send_res = reqwest::blocking::Client::new()
        .post(&send_url)
        .query(&[("alt", "json")])
        .bearer_auth(access_token)
        .json(&body)
        .timeout(std::time::Duration::from_secs(15))
        .send()?;
    if !send_res.status().is_success() {
        let status = send_res.status();
        let err = send_res.text().unwrap_or_else(|_| "unknown".into());
        return Err(format!("Gmail API {}: {}", status, err).into());
    }
    Ok(())
}

/// f78_resend = send via Resend. Skips if RESEND_API_KEY unset.
fn f78_resend(to: &str, full_name: &str, ref_id: &str) {
    let api_key = match std::env::var("RESEND_API_KEY") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            tracing::debug!("RESEND_API_KEY not set, skipping waiver email");
            return;
        }
    };
    let from = std::env::var("RESEND_FROM")
        .unwrap_or_else(|_| "OakilyDokily <noreply@oakilydokily.com>".into());

    let body = serde_json::json!({
        "from": from,
        "to": [to],
        "subject": "Your waiver is confirmed — OakilyDokily",
        "html": format!(
            r#"<p>Hi {},</p><p>Thank you for signing the OakilyDokily service waiver. We've received and recorded your signature.</p><p><strong>Reference ID:</strong> <code>{}</code></p><p>We keep a copy on file for 7 years. If you ever need a copy, just reach out.</p><p>We look forward to caring for your pets.</p><p>— Kaylie &amp; the OakilyDokily team</p>"#,
            f79(full_name),
            f79(ref_id)
        )
    });

    let client = reqwest::blocking::Client::new();
    match client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .timeout(std::time::Duration::from_secs(10))
        .send()
    {
        Ok(r) if r.status().is_success() => {
            tracing::info!("Waiver confirmation email sent to {}", to)
        }
        Ok(r) => tracing::warn!("Resend failed {}: {:?}", r.status(), r.text().ok()),
        Err(e) => tracing::warn!("Resend request failed: {}", e),
    }
}

/// f79 = html_escape (email body)
fn f79(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
