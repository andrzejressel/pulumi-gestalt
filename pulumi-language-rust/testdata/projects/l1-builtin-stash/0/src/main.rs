use anyhow::Result;
fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}
fn pulumi_main(ctx: &pulumi_gestalt_rust::Context) -> Result<()> {
    let myStash = pulumi_gestalt_rust::resources::stash::create(
        &ctx,
        "myStash",
        pulumi_gestalt_rust::resources::stash::StashArgs::builder()
            .input(
                pulumi_gestalt_rust::pulumi_any!(
                    { "" : false, "key" : [("value"), ("s")] }
                ),
            )
            .build_struct(),
    );
    ctx.add_export("stashInput", &myStash.input);
    ctx.add_export("stashOutput", &myStash.output);
    Ok(())
}
