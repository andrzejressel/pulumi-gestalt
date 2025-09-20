use anyhow::{bail, Result};
use pulumi_gestalt_rust::Context;

#[cfg(target_arch = "wasm32")]
pulumi_gestalt_rust::pulumi_main!();
fn pulumi_main(_: &Context) -> Result<()> {
    bail!("Important error message")
}
