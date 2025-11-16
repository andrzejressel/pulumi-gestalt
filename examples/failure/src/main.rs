use anyhow::{Result, bail};
use pulumi_gestalt_rust::Context;

pulumi_gestalt_rust::pulumi_main!();
fn pulumi_main(_: &Context) -> Result<()> {
    bail!("Important error message")
}
