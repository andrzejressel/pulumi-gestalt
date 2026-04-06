use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aList = context
        .require_config_deserialize::<Vec<String>>(None, "aList")
        .expect("Expected config [aList] to exist");
    let singleOrNoneList = context
        .require_config_deserialize::<Vec<String>>(None, "singleOrNoneList")
        .expect("Expected config [singleOrNoneList] to exist");
    let aString = context
        .require_config(None, "aString")
        .expect("Expected config [aString] to exist");
    pulumi_gestalt_rust::add_export(
        "elementOutput1",
        &context
            .new_output(
                &pulumi_gestalt_rust::stdlib::element(&aList, 1)
                    .expect("Element should exist"),
            ),
    );
    pulumi_gestalt_rust::add_export(
        "elementOutput2",
        &context
            .new_output(
                &pulumi_gestalt_rust::stdlib::element(&aList, 2)
                    .expect("Element should exist"),
            ),
    );
    pulumi_gestalt_rust::add_export(
        "joinOutput",
        &context.new_output(&pulumi_gestalt_rust::stdlib::join("|", &aList)),
    );
    pulumi_gestalt_rust::add_export(
        "lengthOutput",
        &context.new_output(&pulumi_gestalt_rust::stdlib::length(&aList)),
    );
    pulumi_gestalt_rust::add_export(
        "splitOutput",
        &context.new_output(&pulumi_gestalt_rust::stdlib::split("-", aString)),
    );
    pulumi_gestalt_rust::add_export(
        "singleOrNoneOutput",
        &context
            .new_output(
                &vec!(
                    pulumi_gestalt_rust::stdlib::single_or_none(singleOrNoneList)
                    .expect("Should get first element")
                ),
            ),
    );
    Ok(())
}
