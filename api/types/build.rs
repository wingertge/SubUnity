use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .out_dir("src/proto")
        .type_attribute(".", "#[derive(Serialize, Deserialize)]")
        .type_attribute(".", r#"#[serde(rename_all = "camelCase")]"#)
        //.type_attribute(".", "#[derive(Debug)]")
        .compile(&[
            "protos/user.proto",
            "protos/subtitles.proto"
        ], &["protos"])?;
    Ok(())
}
