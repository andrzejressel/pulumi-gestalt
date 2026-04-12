use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("stackOutput", &(&ctx.get_stack()).to_string());
    ctx.add_export("projectOutput", &(&ctx.get_project()).to_string());
    ctx.add_export("organizationOutput", &(&ctx.get_organization()).to_string());
    Ok(())
}
