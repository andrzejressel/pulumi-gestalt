use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    context.add_export("output_true", &true);
    context.add_export("output_false", &false);
    Ok(())
}
