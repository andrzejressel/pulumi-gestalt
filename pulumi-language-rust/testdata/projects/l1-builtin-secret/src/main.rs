use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aSecret = ctx
        .require_config_secret(None, "aSecret")
        .expect("Expected config [aSecret] to exist");
    let notSecret = ctx
        .require_config(None, "notSecret")
        .expect("Expected config [notSecret] to exist");
    ctx.add_export("roundtripSecret", &aSecret);
    ctx.add_export("roundtripNotSecret", &notSecret);
    ctx.add_export("double", &aSecret.secret());
    ctx.add_export("open", &aSecret.unsecret());
    ctx.add_export("close", &ctx.new_secret(&notSecret));
    Ok(())
}
