use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let ref_ = pulumi_gestalt_rust::resources::stack_reference::create(
        &ctx,
        "ref",
        pulumi_gestalt_rust::resources::stack_reference::StackReferenceArgs::builder()
            .name("organization/other/dev")
            .build_struct(),
    );
    ctx.add_export("plain", &ref_.get_output(&"plain"));
    ctx.add_export("secret", &ref_.get_output(&"secret"));
    Ok(())
}
