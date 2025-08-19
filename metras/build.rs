use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../schemas/proto/credential_payload.proto"],
        &["../schemas/proto"],
    )?;
    Ok(())
}
