fn main() -> Result<(), Box<dyn std::error::Error>> {
    pulumi_gestalt_build::generate_from_schema("../../pulumi-test/schema.json".as_ref())?;
    Ok(())
}
