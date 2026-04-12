use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("output_true", &true);
    ctx.add_export("output_false", &false);
    Ok(())
}
