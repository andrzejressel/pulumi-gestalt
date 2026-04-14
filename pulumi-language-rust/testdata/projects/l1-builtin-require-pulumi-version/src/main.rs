use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let version = ctx
        .require_config(None, "version")
        .expect("Expected config [version] to exist");
    ctx.require_pulumi_version(&version)
        .expect("Failed to require Pulumi version");
    Ok(())
}
