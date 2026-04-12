use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aMap = ctx
        .require_config_deserialize::<
            std::collections::BTreeMap<String, String>,
        >(None, "aMap")
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
