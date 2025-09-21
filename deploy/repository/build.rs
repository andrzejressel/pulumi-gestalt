fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_build::generate("github", "5.26.0")?;
    Ok(())
}
