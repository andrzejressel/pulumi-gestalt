use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(context: &pulumi_gestalt_rust::Context) -> Result<()> {
    let input = context
        .require_config(None, "input")
        .expect("Expected config [input] to exist");
    let hash = pulumi_gestalt_rust::stdlib::sha1(input);
    pulumi_gestalt_rust::add_export("hash", &context.new_output(&hash));
    Ok(())
}
