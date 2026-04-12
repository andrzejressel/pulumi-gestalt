use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aList = ctx
        .require_config_deserialize::<Vec<String>>(None, "aList")
        .expect("Expected config [aList] to exist");
    let singleOrNoneList = ctx
        .require_config_deserialize::<Vec<String>>(None, "singleOrNoneList")
        .expect("Expected config [singleOrNoneList] to exist");
    let aString = ctx
        .require_config(None, "aString")
        .expect("Expected config [aString] to exist");
    ctx.add_export(
        "elementOutput1",
        &pulumi_gestalt_rust::stdlib::element(&aList, 1).expect("Element should exist"),
    );
    ctx.add_export(
        "elementOutput2",
        &pulumi_gestalt_rust::stdlib::element(&aList, 2).expect("Element should exist"),
    );
    ctx.add_export("joinOutput", &pulumi_gestalt_rust::stdlib::join("|", &aList));
    ctx.add_export("lengthOutput", &pulumi_gestalt_rust::stdlib::length(&aList));
    ctx.add_export("splitOutput", &pulumi_gestalt_rust::stdlib::split("-", aString));
    ctx.add_export(
        "singleOrNoneOutput",
        &vec!(
            pulumi_gestalt_rust::stdlib::single_or_none(singleOrNoneList)
            .expect("Should get first element")
        ),
    );
    Ok(())
}
