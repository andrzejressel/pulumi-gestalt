#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamDestinationConfigGcsDestinationConfigJsonFileFormat {
    /// Compression of the loaded JSON file.
    /// Possible values are: `NO_COMPRESSION`, `GZIP`.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Option<String>,
    /// The schema file format along JSON data files.
    /// Possible values are: `NO_SCHEMA_FILE`, `AVRO_SCHEMA_FILE`.
    #[builder(into)]
    #[serde(rename = "schemaFileFormat")]
    pub r#schema_file_format: Option<String>,
}
