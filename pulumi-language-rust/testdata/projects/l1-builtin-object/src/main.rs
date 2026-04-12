use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aMap = context
        .require_config_deserialize::<
            std::collections::BTreeMap<String, String>,
        >(None, "aMap")
        .expect("Expected config [aMap] to exist");
    context.add_export("entriesOutput", &pulumi_gestalt_rust::stdlib::entries(&aMap));
    context
        .add_export(
            "lookupOutput",
            &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyPresent", "default"),
        );
    context
        .add_export(
            "lookupOutputDefault",
            &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyMissing", "default"),
        );
    Ok(())
}
