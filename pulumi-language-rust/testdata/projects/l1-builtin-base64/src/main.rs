use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let input = ctx
        .require_config(None, "input")
        .expect("Expected config [input] to exist");
    let bytes = pulumi_gestalt_rust::stdlib::from_base64(input)
        .expect("Fail to convert from base64");
    ctx.add_export("data", &bytes);
    ctx.add_export("roundtrip", &pulumi_gestalt_rust::stdlib::to_base64(bytes));
    Ok(())
}
