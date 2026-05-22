use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let names = ctx
        .require_config_deserialize::<Vec<Option<String>>>(None, "names")
        .unwrap_or(vec!(None, Some("hello".to_string()), None));
    ctx.add_export("namesLength", &pulumi_gestalt_rust::stdlib::length(&names));
    Ok(())
}
