use anyhow::Result;
use std::fs;

pub(crate) fn regenerate_proto() -> Result<()> {
    let pulumi_proto_dir = "external/pulumi/proto";

    let out_location = "crates/proto/src";
    let pulumi_location = format!("{}/pulumi", out_location);

    fs::create_dir_all(&pulumi_location)?;

    tonic_prost_build::configure()
        .build_transport(true)
        .build_client(true)
        .build_server(true)
        .out_dir(pulumi_location)
        .compile_protos(
            &[
                format!("{}/pulumi/plugin.proto", pulumi_proto_dir),
                format!("{}/pulumi/engine.proto", pulumi_proto_dir),
                format!("{}/pulumi/resource.proto", pulumi_proto_dir),
            ],
            &[pulumi_proto_dir.to_string()],
        )?;

    Ok(())
}
