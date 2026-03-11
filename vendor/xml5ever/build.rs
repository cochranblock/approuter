fn main() {
    println!("cargo::rustc-check-cfg=cfg(trace_tokenizer)");
    println!("cargo::rustc-check-cfg=cfg(for_c)");
}
