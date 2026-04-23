use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let a = ctx
        .require_config_deserialize::<f64>(None, "a")
        .expect("Expected config [a] to exist");
    let b = ctx
        .require_config_deserialize::<f64>(None, "b")
        .expect("Expected config [b] to exist");
    let c = ctx
        .require_config_deserialize::<i64>(None, "c")
        .expect("Expected config [c] to exist");
    let d = ctx
        .require_config_deserialize::<i64>(None, "d")
        .expect("Expected config [d] to exist");
    ctx.add_export("maxResult", &a.max(b));
    ctx.add_export("minResult", &a.min(b));
    ctx.add_export("intMaxResult", &c.max(d));
    ctx.add_export("intMinResult", &c.min(d));
    Ok(())
}
