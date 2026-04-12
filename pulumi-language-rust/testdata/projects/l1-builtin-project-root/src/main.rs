use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    context
        .add_export("rootDirectoryOutput", &(&context.get_root_directory()).to_string());
    Ok(())
}
