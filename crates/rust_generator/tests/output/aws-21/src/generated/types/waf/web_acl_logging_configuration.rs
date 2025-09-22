#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclLoggingConfiguration {
    /// Amazon Resource Name (ARN) of Kinesis Firehose Delivery Stream
    #[builder(into)]
    #[serde(rename = "logDestination")]
    pub r#log_destination: String,
    /// Configuration block containing parts of the request that you want redacted from the logs. Detailed below.
    #[builder(into)]
    #[serde(rename = "redactedFields")]
    pub r#redacted_fields: Option<Box<super::super::types::waf::WebAclLoggingConfigurationRedactedFields>>,
}
