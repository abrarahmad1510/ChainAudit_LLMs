use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::compile_protos("../shared/proto/auditor.proto")?;

    let protos = &[
        "../shared/proto/trillian/trillian.proto",
        "../shared/proto/trillian/trillian_log_api.proto",
        "../shared/proto/trillian/trillian_admin_api.proto",
        "../shared/proto/google/rpc/status.proto",
    ];

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("trillian_descriptor.bin"))
        .build_client(true)
        .build_server(false)
        .compile(protos, &["../shared/proto/trillian", "../shared/proto"])?;

    Ok(())
}
