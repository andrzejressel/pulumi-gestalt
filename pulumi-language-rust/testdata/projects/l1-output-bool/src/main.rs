use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    pulumi_gestalt_rust::add_export("output_true", &context.new_output(&true));
    pulumi_gestalt_rust::add_export("output_false", &context.new_output(&false));
    Ok(())
}
