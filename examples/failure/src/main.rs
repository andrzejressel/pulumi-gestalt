use anyhow::{Result, bail};
use pulumi_gestalt_rust::Context;

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

fn pulumi_main(_: &Context) -> Result<()> {
    bail!("Important error message")
}
