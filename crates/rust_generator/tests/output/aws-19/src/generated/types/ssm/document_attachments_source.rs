#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentAttachmentsSource {
    /// The key of a key-value pair that identifies the location of an attachment to the document. Valid values: `SourceUrl`, `S3FileUrl`, `AttachmentReference`.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// The name of the document attachment file.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of a key-value pair that identifies the location of an attachment to the document. The argument format is a list of a single string that depends on the type of key you specify - see the [API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_AttachmentsSource.html) for details.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
