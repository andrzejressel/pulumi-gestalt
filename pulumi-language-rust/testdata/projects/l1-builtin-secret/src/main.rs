use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aSecret = context
        .require_config_secret(None, "aSecret")
        .expect("Expected config [aSecret] to exist");
    let notSecret = context
        .require_config(None, "notSecret")
        .expect("Expected config [notSecret] to exist");
    context.add_export("roundtripSecret", &aSecret);
    context.add_export("roundtripNotSecret", &notSecret);
    context.add_export("double", &aSecret.secret());
    context.add_export("open", &aSecret.unsecret());
    context.add_export("close", &context.new_secret(&notSecret));
    Ok(())
}
