// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f70=whyyoulying_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests.

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ok = exopack::triple_sims::f60(|| async { whyyoulying::tests::f30() == 0 }).await;
    std::process::exit(if ok { 0 } else { 1 });
}
