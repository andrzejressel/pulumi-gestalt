use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    context.add_export("stackOutput", &(&context.get_stack()).to_string());
    context.add_export("projectOutput", &(&context.get_project()).to_string());
    context.add_export("organizationOutput", &(&context.get_organization()).to_string());
    Ok(())
}
