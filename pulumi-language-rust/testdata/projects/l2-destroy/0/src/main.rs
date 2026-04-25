use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aresource = pulumi_simple::resource::create(
        &ctx,
        "aresource",
        pulumi_simple::resource::ResourceArgs::builder().value(true).build_struct(),
    );
    let other = pulumi_simple::resource::create(
        &ctx,
        "other",
        pulumi_simple::resource::ResourceArgs::builder().value(true).build_struct(),
    );
    Ok(())
}
