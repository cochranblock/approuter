//! Build script: compiles Cap'n Proto schemas when daemon feature is enabled.
//! Requires `capnp` compiler: brew install capnp / apt install capnproto

fn main() {
    #[cfg(feature = "daemon")]
    {
        capnpc::CompilerCommand::new()
            .src_prefix("schema")
            .file("schema/kova_protocol.capnp")
            .run()
            .expect("Cap'n Proto schema compilation failed. Install capnp: brew install capnp");
    }
}
