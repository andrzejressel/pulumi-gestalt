use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aNumber = ctx
        .require_config_secret_deserialize::<f64>(None, "aNumber")
        .expect("Expected config [aNumber] to exist");
    ctx.add_export("roundtrip", &aNumber);
    ctx.add_export("theSecretNumber", &aNumber.map(|aNumber| (aNumber + 1.25)));
    Ok(())
}
