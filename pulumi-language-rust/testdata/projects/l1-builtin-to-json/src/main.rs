use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![], pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aString = ctx
        .require_config(None, "aString")
        .expect("Expected config [aString] to exist");
    let aNumber: f64 = ctx
        .require_config_deserialize(None, "aNumber")
        .expect("Expected config [aNumber] to exist");
    let aList: Vec<String> = ctx
        .require_config_deserialize(None, "aList")
        .expect("Expected config [aList] to exist");
    let aSecret = ctx
        .require_config_secret(None, "aSecret")
        .expect("Expected config [aSecret] to exist");
    ctx.add_export("stringOutput", &pulumi_gestalt_rust::stdlib::to_json(&aString));
    ctx.add_export("numberOutput", &pulumi_gestalt_rust::stdlib::to_json(&aNumber));
    ctx.add_export("boolOutput", &pulumi_gestalt_rust::stdlib::to_json(&true));
    ctx.add_export(
        "arrayOutput",
        &pulumi_gestalt_rust::stdlib::to_json(&vec!["x", "y", "z"]),
    );
    ctx.add_export(
        "objectOutput",
        &pulumi_gestalt_rust::stdlib::to_json(
            &pulumi_gestalt_rust::pulumi_any!(
                { "count" : 1, "key" : (("value").clone()) }
            ),
        ),
    );
    let nestedObject = pulumi_gestalt_rust::pulumi_any!(
        { "a_secret" : ((aSecret).clone()), "anObject" : { "items" : ((aList).clone()),
        "name" : ((aString).clone()) } }
    );
    ctx.add_export("nestedOutput", &pulumi_gestalt_rust::stdlib::to_json(&nestedObject));
    Ok(())
}
