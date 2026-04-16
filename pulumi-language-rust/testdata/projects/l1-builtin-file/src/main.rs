use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let fileContent = pulumi_gestalt_rust::stdlib::read_file("testfile.txt")
        .expect("Failed to read file");
    let fileB64 = pulumi_gestalt_rust::stdlib::filebase64("testfile.txt")
        .expect("Failed to read file as base64");
    let fileSha = pulumi_gestalt_rust::stdlib::filebase64sha256("testfile.txt")
        .expect("Failed to compute file sha256");
    ctx.add_export("fileContent", &fileContent);
    ctx.add_export("fileB64", &fileB64);
    ctx.add_export("fileSha", &fileSha);
    Ok(())
}
