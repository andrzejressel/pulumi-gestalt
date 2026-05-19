use anyhow::Result;
use pulumi_gestalt_rust::resources::stash;
use pulumi_gestalt_rust::resources::stash::StashArgs;
use pulumi_gestalt_rust::{Context, ToPulumiAny, add_export};

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

fn pulumi_main(ctx: &Context) -> Result<()> {
    let stash_input = "stash-value".to_pulumi_any();
    let stash = stash::create(
        ctx,
        "example_stash",
        StashArgs::builder().input(stash_input).build_struct(),
    );

    ctx.add_export("stash_input", &stash.input);
    ctx.add_export("stash_output", &stash.output);
    ctx.add_export("stash_id", &stash.id);
    ctx.add_export("stash_urn", &stash.urn);
    Ok(())
}
