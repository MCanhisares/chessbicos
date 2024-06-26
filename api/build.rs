use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = ["./proto/chessgame.proto", "./proto/auth.proto"];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("chessbicos_descriptor.bin"))
        .compile(&proto_files, &["proto"])?;

    Ok(())
}
