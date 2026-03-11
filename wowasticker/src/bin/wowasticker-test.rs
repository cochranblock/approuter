//! f131=wowasticker_test. Clippy + TRIPLE SIMS via exopack f61_with_args.

use std::path::Path;
use std::process::Command;

fn main() {
    let project = Path::new(env!("CARGO_MANIFEST_DIR"));

    // 1. cargo clippy (--no-default-features: lib only, no Dioxus/GTK)
    println!("wowasticker-test: cargo clippy...");
    let out = Command::new("cargo")
        .args(["clippy", "--no-default-features", "--", "-D", "warnings"])
        .current_dir(project)
        .output()
        .expect("spawn clippy");
    if !out.status.success() {
        eprintln!("clippy failed:\n{}", String::from_utf8_lossy(&out.stderr));
        std::process::exit(1);
    }

    // 2. TRIPLE SIMS — cargo test 3x via exopack (--no-default-features)
    println!("wowasticker-test: TRIPLE SIMS (cargo test 3x)...");
    let (ok, stderr) = exopack::triple_sims::f61_with_args(project, 3, &["--no-default-features"]);
    if !ok {
        eprintln!("{}", stderr);
        std::process::exit(1);
    }

    println!("wowasticker-test: all checks passed");
}
