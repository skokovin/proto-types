use std::env;
use std::io::Result;

fn main() -> Result<()> {

    println!("cargo:rerun-if-changed=proto");

    tonic_prost_build::compile_protos("proto/radar.proto");
    tonic_prost_build::compile_protos("proto/geyser.proto");
    Ok(())
}

/*use std::env;
use std::io::Result;

fn main() -> Result<()> {

    println!("cargo:rerun-if-changed=proto");


    if env::var("CARGO_FEATURE_INGEST").is_ok() {
        tonic_prost_build::compile_protos("proto/geyser.proto");
    }


    if env::var("CARGO_FEATURE_RADAR").is_ok() {
        tonic_prost_build::compile_protos("proto/radar.proto");
    }

    Ok(())
}*/
