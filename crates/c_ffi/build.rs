extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable must be set");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Failed to load cbindgen.toml configuration file");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("pulumi_gestalt.h");
}
