use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aresource = test::create(
        &ctx,
        "aresource",
        test::testArgs::builder()
            .value(pulumi_gestalt_rust::pulumi_any!(true))
            .build_struct(),
    );
    let other = test::create(
        &ctx,
        "other",
        test::testArgs::builder()
            .value(pulumi_gestalt_rust::pulumi_any!(true))
            .build_struct(),
    );
    Ok(())
}
