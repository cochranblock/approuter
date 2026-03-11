// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f31=rogue_repo_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests.

use rogue_repo::tests;

#[tokio::main]
async fn main() {
    let ok = exopack::triple_sims::f60(|| async { tests::f30().await }).await;
    std::process::exit(if ok { 0 } else { 1 });
}
