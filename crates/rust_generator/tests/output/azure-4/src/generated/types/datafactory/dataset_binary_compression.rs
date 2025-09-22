#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatasetBinaryCompression {
    /// The level of compression. Possible values are `Fastest` and `Optimal`.
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    /// The type of compression used during transport. Possible values are `BZip2`, `Deflate`, `GZip`, `Tar`, `TarGZip` and `ZipDeflate`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
