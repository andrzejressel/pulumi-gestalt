use anyhow::{Result, bail};
use pulumi_gestalt_rust::Context;

#[cfg(target_arch = "wasm32")]
pulumi_gestalt_rust::pulumi_main!();
#[allow(dead_code)]
fn pulumi_main(_: &Context) -> Result<()> {
    bail!("Important error message")
}
