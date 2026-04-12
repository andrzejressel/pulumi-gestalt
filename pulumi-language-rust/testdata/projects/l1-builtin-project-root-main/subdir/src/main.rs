use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("rootDirectoryOutput", &(&ctx.get_root_directory()).to_string());
    ctx.add_export(
        "workingDirectoryOutput",
        &pulumi_gestalt_rust::stdlib::cwd().expect("Failed to get current directory"),
    );
    Ok(())
}
