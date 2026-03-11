// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! exopack CLI: live-demo for -test binaries. Build and run with streaming output.

use std::path::PathBuf;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("exopack: testing augmentation");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  exopack live-demo ./wowasticker --no-default-features --features tests");
        eprintln!("  exopack live-demo ./oakilydokily oakilydokily-test --features tests");
        exit(1);
    }

    let sub = &args[1];
    if sub != "live-demo" {
        eprintln!("Unknown subcommand: {}. Use: live-demo", sub);
        exit(1);
    }

    if args.len() < 3 {
        eprintln!("live-demo requires <project_dir>");
        eprintln!("  exopack live-demo <project_dir> [bin_name] [cargo_args...]");
        exit(1);
    }

    let project_dir = PathBuf::from(&args[2]);
    let (bin_name, cargo_args): (String, Vec<&str>) = if args.len() >= 4 && !args[3].starts_with('-') {
        (args[3].clone(), args[4..].iter().map(|s| s.as_str()).collect())
    } else {
        match exopack::triple_sims::f63_discover_test_bin(&project_dir) {
            Some(b) => (b, args[3..].iter().map(|s| s.as_str()).collect()),
            None => {
                eprintln!("No -test binary found in Cargo.toml. Specify bin_name explicitly.");
                exit(1);
            }
        }
    };

    if !project_dir.join("Cargo.toml").exists() {
        eprintln!("Cargo.toml not found in {}", project_dir.display());
        exit(1);
    }

    println!("exopack live-demo: building and running {} in {}...", bin_name, project_dir.display());
    match exopack::triple_sims::f62_live_demo(&project_dir, bin_name.as_str(), &cargo_args) {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(e) => {
            eprintln!("exopack live-demo: {}", e);
            exit(1);
        }
    }
}
