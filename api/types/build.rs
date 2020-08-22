use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["protos/user.proto"], &["protos"])?;
    Ok(())
}
