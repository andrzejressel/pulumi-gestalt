use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let input = ctx
        .require_config(None, "input")
        .expect("Expected config [input] to exist");
    let hash = pulumi_gestalt_rust::stdlib::sha1(input);
    ctx.add_export("hash", &hash);
    Ok(())
}
