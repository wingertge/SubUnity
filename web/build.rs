use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    use ructe::Ructe;

    let mut ructe = Ructe::new("src".into())?;
    ructe.statics()?.add_files("assets")?;
    ructe.compile_templates("templates")?;
    Ok(())
}