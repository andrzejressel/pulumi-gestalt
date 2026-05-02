use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let res = pulumi_secret::resource::create(
        &ctx,
        "res",
        pulumi_secret::resource::ResourceArgs::builder()
            .private("closed")
            .public("open")
            .private_data(
                pulumi_gestalt_rust::pulumi_any!(
                    { "private" : ("closed"), "public" : ("open") }
                ),
            )
            .public_data(
                pulumi_gestalt_rust::pulumi_any!(
                    { "private" : ("closed"), "public" : ("open") }
                ),
            )
            .build_struct(),
    );
    Ok(())
}
