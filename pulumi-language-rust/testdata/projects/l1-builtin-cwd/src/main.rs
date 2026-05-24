use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run_with_packages(vec![], pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export(
        "cwdOutput",
        &pulumi_gestalt_rust::stdlib::cwd().expect("Failed to get current directory"),
    );
    Ok(())
}
