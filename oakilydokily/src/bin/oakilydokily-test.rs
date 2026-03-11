// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f70=oakilydokily_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests. f53=screenshot. f62=console check.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = oakilydokily::screenshot::f53().await;
    let console_errors = oakilydokily::screenshot::f62().await;
    if !console_errors.is_empty() {
        for e in &console_errors {
            eprintln!("console: {}", e);
        }
        std::process::exit(1);
    }
    let ok = exopack::triple_sims::f60(|| async {
        oakilydokily::tests::f30().await == 0
    })
    .await;
    std::process::exit(if ok { 0 } else { 1 });
}
