use anyhow::Context;
use anyhow::Result;
use prost::Message;
use pulumi_gestalt_schema::model::Package;
mod converter;

pub fn convert_to_protobuf(package: &Package) -> Result<Vec<u8>> {
    let proto =
        converter::package_to_proto(package).context("Failed to convert package to protobuf")?;

    Ok(proto.encode_to_vec())
}
