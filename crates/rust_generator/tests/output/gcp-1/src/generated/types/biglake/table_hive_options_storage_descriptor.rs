#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableHiveOptionsStorageDescriptor {
    /// The fully qualified Java class name of the input format.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Option<String>,
    /// Cloud Storage folder URI where the table data is stored, starting with "gs://".
    #[builder(into)]
    #[serde(rename = "locationUri")]
    pub r#location_uri: Option<String>,
    /// The fully qualified Java class name of the output format.
    #[builder(into)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Option<String>,
}
