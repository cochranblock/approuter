//! whyyoulying CLI — proactive labor category and ghost billing detection

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use whyyoulying::{Alert, Config, GhostDetector, Ingest, LaborDetector};

#[derive(Parser)]
#[command(name = "whyyoulying")]
#[command(about = "Proactive Labor Category Fraud and Ghost Billing detection")]
#[command(version)]
struct Cli {
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    #[arg(long, global = true)]
    data_path: Option<PathBuf>,

    #[arg(long, global = true, value_parser = clap::value_parser!(f64))]
    threshold: Option<f64>,

    #[arg(long, global = true, value_parser = clap::value_parser!(u8).range(0..=100), help = "Min confidence 0-100 (S4 false-positive control)")]
    min_confidence: Option<u8>,

    #[arg(long, global = true, help = "DoD nexus: filter by agency (e.g. DoD, Army)")]
    agency: Option<String>,

    #[arg(long, global = true, help = "DoD nexus: filter by CAGE code")]
    cage_code: Option<String>,

    #[arg(long, short, global = true, default_value = "json", value_enum)]
    output: OutputFormat,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Json,
    Csv,
}

#[derive(Subcommand)]
enum Commands {
    /// Run labor + ghost detection, output alerts (default)
    Run,
    /// Load and validate data only
    Ingest {
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Export GAGAS referral package or FBI case-opening docs
    ExportReferral {
        #[arg(long)]
        path: Option<PathBuf>,
        #[arg(long, default_value_t = false, help = "FBI case-opening format (AG Guidelines)")]
        fbi: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        None | Some(Commands::Run) => run(&cli),
        Some(Commands::Ingest { path }) => cmd_ingest(&cli, path.as_deref()),
        Some(Commands::ExportReferral { path, fbi }) => cmd_export_referral(&cli, path.as_deref(), *fbi),
    };

    match result {
        Ok(exit_code) => std::process::exit(exit_code),
        Err(e) => {
            eprintln!("error: {e:?}");
            std::process::exit(2);
        }
    }
}

fn load_config(cli: &Cli) -> Result<Config> {
    let mut cfg = if let Some(ref p) = cli.config {
        Config::load_from_path(p)?
    } else {
        Config::load()?
    };
    cfg.apply_cli_overrides(
        cli.data_path.as_ref().map(|p| p.to_string_lossy().into_owned()),
        cli.threshold,
        cli.min_confidence,
        cli.agency.clone(),
        cli.cage_code.clone(),
    )?;
    Ok(cfg)
}

fn run(cli: &Cli) -> Result<i32> {
    let config = load_config(cli)?;
    let data_path = config
        .data_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("--data-path or config data_path required"))?;

    eprintln!("loading data from {}", data_path.display());
    let ds = Ingest::load_from_path(&data_path)?;
    eprintln!(
        "loaded {} contracts, {} employees, {} labor charges, {} billing records",
        ds.contracts.len(),
        ds.employees.len(),
        ds.labor_charges.len(),
        ds.billing_records.len()
    );

    let labor = LaborDetector::new(config.labor_variance_threshold_pct);
    let ghost = GhostDetector::new();
    let labor_alerts = labor.run(&ds);
    let ghost_alerts = ghost.run(&ds);
    let mut alerts: Vec<Alert> = labor_alerts
        .into_iter()
        .chain(ghost_alerts)
        .collect();

    let nexus_ids = ds.nexus_contract_ids(
        config.filter_agency.as_deref(),
        config.filter_cage_code.as_deref(),
    );
    alerts.retain(|a| {
        a.confidence >= config.min_confidence
            && a.contract_id
                .as_ref()
                .is_none_or(|id| nexus_ids.contains(id.as_str()))
    });

    match cli.output {
        OutputFormat::Json => {
            let out = serde_json::to_string_pretty(&alerts)?;
            println!("{out}");
        }
        OutputFormat::Csv => {
            println!("fraud_type,rule_id,severity,confidence,summary,contract_id,employee_id,cage_code,agency,timestamp");
            for a in &alerts {
                println!(
                    "{},{},{},{},{},{},{},{},{},{}",
                    a.fraud_type,
                    a.rule_id,
                    a.severity,
                    a.confidence,
                    escape_csv(&a.summary),
                    a.contract_id.as_deref().unwrap_or(""),
                    a.employee_id.as_deref().unwrap_or(""),
                    a.cage_code.as_deref().unwrap_or(""),
                    a.agency.as_deref().unwrap_or(""),
                    a.timestamp.as_deref().unwrap_or("")
                );
            }
        }
    }

    Ok(if alerts.is_empty() { 0 } else { 1 })
}

fn cmd_ingest(cli: &Cli, path: Option<&std::path::Path>) -> Result<i32> {
    let config = load_config(cli)?;
    let p = path
        .map(PathBuf::from)
        .or_else(|| config.data_path.as_ref().map(PathBuf::from))
        .ok_or_else(|| anyhow::anyhow!("--path or --data-path required"))?;
    let ds = Ingest::load_from_path(&p)?;
    eprintln!(
        "ingested: {} contracts, {} employees, {} labor charges, {} billing records",
        ds.contracts.len(),
        ds.employees.len(),
        ds.labor_charges.len(),
        ds.billing_records.len()
    );
    Ok(0)
}

fn cmd_export_referral(cli: &Cli, path: Option<&std::path::Path>, fbi_format: bool) -> Result<i32> {
    let config = load_config(cli)?;
    let data_path = config
        .data_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("--data-path required for export-referral"))?;
    let ds = Ingest::load_from_path(&data_path)?;
    let labor = LaborDetector::new(config.labor_variance_threshold_pct);
    let ghost = GhostDetector::new();
    let mut alerts: Vec<Alert> = labor
        .run(&ds)
        .into_iter()
        .chain(ghost.run(&ds))
        .collect();

    let nexus_ids = ds.nexus_contract_ids(
        config.filter_agency.as_deref(),
        config.filter_cage_code.as_deref(),
    );
    alerts.retain(|a| {
        a.confidence >= config.min_confidence
            && a.contract_id
                .as_ref()
                .is_none_or(|id| nexus_ids.contains(id.as_str()))
    });

    let out = if fbi_format {
        serde_json::to_string_pretty(&whyyoulying::export::fbi_case_opening(&alerts))?
    } else {
        serde_json::to_string_pretty(&whyyoulying::export::referral_package(&alerts))?
    };

    if let Some(p) = path {
        std::fs::write(p, &out)?;
        eprintln!("wrote {} package to {}", if fbi_format { "FBI case-opening" } else { "GAGAS referral" }, p.display());
    } else {
        println!("{out}");
    }
    Ok(if alerts.is_empty() { 0 } else { 1 })
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
