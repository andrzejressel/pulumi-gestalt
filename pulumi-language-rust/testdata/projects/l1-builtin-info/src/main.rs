use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    pulumi_gestalt_rust::add_export(
        "stackOutput",
        &context.new_output(&(&context.get_stack()).to_string()),
    );
    pulumi_gestalt_rust::add_export(
        "projectOutput",
        &context.new_output(&(&context.get_project()).to_string()),
    );
    pulumi_gestalt_rust::add_export(
        "organizationOutput",
        &context.new_output(&(&context.get_organization()).to_string()),
    );
    Ok(())
}
