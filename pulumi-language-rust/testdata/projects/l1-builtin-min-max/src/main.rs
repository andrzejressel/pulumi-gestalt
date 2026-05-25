use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![], pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let a: f64 = ctx
        .require_config_deserialize(None, "a")
        .expect("Expected config [a] to exist");
    let b: f64 = ctx
        .require_config_deserialize(None, "b")
        .expect("Expected config [b] to exist");
    let c: i64 = ctx
        .require_config_deserialize(None, "c")
        .expect("Expected config [c] to exist");
    let d: i64 = ctx
        .require_config_deserialize(None, "d")
        .expect("Expected config [d] to exist");
    ctx.add_export("maxResult", &a.max(b));
    ctx.add_export("minResult", &a.min(b));
    ctx.add_export("intMaxResult", &c.max(d));
    ctx.add_export("intMinResult", &c.min(d));
    Ok(())
}
