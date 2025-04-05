use anyhow::Result;
use std::fs;

pub(crate) fn regenerate_proto() -> Result<()> {
    let pulumi_proto_dir = "external/pulumi/proto";
    let pulumi_gestalt_proto_dir = "proto";

    let out_location = "crates/proto/src";
    let full_location = format!("{}/full", out_location);
    let pulumi_gestalt_location = format!("{}/pulumi_gestalt", out_location);

    fs::create_dir_all(&full_location)?;
    fs::create_dir_all(&pulumi_gestalt_location)?;

    tonic_build::configure()
        .build_transport(true)
        .build_client(true)
        .build_server(true)
        .out_dir(full_location)
        .compile_protos(
            &[
                format!("{}/pulumi/plugin.proto", pulumi_proto_dir),
                format!("{}/pulumi/engine.proto", pulumi_proto_dir),
                format!("{}/pulumi/resource.proto", pulumi_proto_dir),
            ],
            &[pulumi_proto_dir],
        )?;

    tonic_build::configure()
        .build_transport(false)
        .build_client(false)
        .build_server(false)
        .out_dir(pulumi_gestalt_location)
        .compile_protos(
            &[
                format!("{}/pulumi_gestalt.proto", pulumi_gestalt_proto_dir)
            ],
            &Vec::<String>::new()
        )?;
    
    Ok(())
}
