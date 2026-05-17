use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aString = ctx
        .require_config(None, "aString")
        .expect("Expected config [aString] to exist");
    ctx.add_export(
        "lengthResult",
        &pulumi_gestalt_rust::stdlib::length_string(&aString),
    );
    ctx.add_export("splitResult", &pulumi_gestalt_rust::stdlib::split("-", &aString));
    ctx.add_export(
        "joinResult",
        &pulumi_gestalt_rust::stdlib::join(
            "|",
            &pulumi_gestalt_rust::stdlib::split("-", &aString),
        ),
    );
    ctx.add_export("interpolateResult", &format!("{}{}", "prefix-", aString));
    Ok(())
}
