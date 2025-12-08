use std::io::Result;

fn main() -> Result<()> {
    // В библиотеке build.rs делает то же самое: компилирует .proto
    // Важно: файл trade.proto теперь должен лежать внутри папки библиотеки: proto_types/proto/trade.proto
    println!("cargo:rerun-if-changed=proto/trade.proto");
    prost_build::compile_protos(&["proto/trade.proto","proto/geyser.proto"], &["proto/"])?;
    Ok(())
}