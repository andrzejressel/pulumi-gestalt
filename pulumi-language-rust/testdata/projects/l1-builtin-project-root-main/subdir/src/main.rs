use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    context
        .add_export("rootDirectoryOutput", &(&context.get_root_directory()).to_string());
    context
        .add_export(
            "workingDirectoryOutput",
            &pulumi_gestalt_rust::stdlib::cwd().expect("Failed to get current directory"),
        );
    Ok(())
}
