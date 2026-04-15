use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    ctx.add_export("empty", &Vec::<String>::new());
    ctx.add_export("strings", &Vec::<String>::new());
    ctx.add_export("numbers", &Vec::<String>::new());
    ctx.add_export("keys", &Vec::<String>::new());
    Ok(())
}
