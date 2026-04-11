use anyhow::Result;
use pulumi_gestalt_rust::*;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let aSecret = context
        .require_config(None, "aSecret")
        .expect("Expected config [aSecret] to exist");
    let notSecret = context
        .require_config(None, "notSecret")
        .expect("Expected config [notSecret] to exist");
    context.add_export("roundtripSecret", &aSecret);
    context.add_export("roundtripNotSecret", &notSecret);
    context.add_export("double", &aSecret.as_secret_output(context));
    context.add_export("open", &aSecret.as_unsecret_output(context));
    context.add_export("close", &notSecret.as_secret_output(context));
    Ok(())
}
