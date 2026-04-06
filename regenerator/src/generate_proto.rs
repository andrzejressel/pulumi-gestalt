use anyhow::Result;
use std::fs;

pub(crate) fn regenerate_proto() -> Result<()> {
    let pulumi_proto_dir = "external/pulumi/proto";
    let language_server_proto_dir = "proto/language_server";

    let out_location = "crates/proto/src";
    let pulumi_location = format!("{}/pulumi", out_location);
    let language_server_location = format!("{}/language_server", out_location);

    fs::create_dir_all(&pulumi_location)?;
    fs::create_dir_all(&language_server_location)?;

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

    tonic_prost_build::configure()
        .build_transport(false)
        .build_client(false)
        .build_server(false)
        .out_dir(&language_server_location)
        .compile_protos(
            &[
                format!("{}/pcl.proto", language_server_proto_dir),
                format!("{}/package.proto", language_server_proto_dir),
            ],
            &[language_server_proto_dir.to_string()],
        )?;

    Ok(())
}
