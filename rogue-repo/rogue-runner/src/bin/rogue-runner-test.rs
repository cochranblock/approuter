// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f115=rogue_runner_test. TRIPLE SIMS via exopack::triple_sims::f61.

fn main() {
    let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let (ok, stderr) = exopack::triple_sims::f61(project, 3);
    if !ok {
        eprintln!("{}", stderr);
        std::process::exit(1);
    }
}
