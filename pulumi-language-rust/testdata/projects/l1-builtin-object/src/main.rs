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
    pulumi_gestalt_rust::add_export(
        "entriesOutput",
        &context.new_output(&pulumi_gestalt_rust::stdlib::entries(&aMap)),
    );
    pulumi_gestalt_rust::add_export(
        "lookupOutput",
        &context
            .new_output(
                &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyPresent", "default"),
            ),
    );
    pulumi_gestalt_rust::add_export(
        "lookupOutputDefault",
        &context
            .new_output(
                &pulumi_gestalt_rust::stdlib::lookup(&aMap, "keyMissing", "default"),
            ),
    );
    Ok(())
}
