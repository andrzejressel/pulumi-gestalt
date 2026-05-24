use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![], pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aMap: std::collections::BTreeMap<String, String> = ctx
        .require_config_deserialize(None, "aMap")
        .expect("Expected config [aMap] to exist");
    ctx.add_export("entriesOutput", &pulumi_gestalt_rust::stdlib::entries(&aMap));
    ctx.add_export(
        "lookupOutput",
        &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyPresent", "default"),
    );
    ctx.add_export(
        "lookupOutputDefault",
        &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyMissing", "default"),
    );
    Ok(())
}
