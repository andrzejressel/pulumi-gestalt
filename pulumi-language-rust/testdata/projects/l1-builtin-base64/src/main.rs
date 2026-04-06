use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let input = context
        .require_config(None, "input")
        .expect("Expected config [input] to exist");
    let bytes = pulumi_gestalt_rust::stdlib::from_base64(input)
        .expect("Fail to convert from base64");
    pulumi_gestalt_rust::add_export("data", &context.new_output(&bytes));
    pulumi_gestalt_rust::add_export(
        "roundtrip",
        &context.new_output(&pulumi_gestalt_rust::stdlib::to_base64(bytes)),
    );
    Ok(())
}
