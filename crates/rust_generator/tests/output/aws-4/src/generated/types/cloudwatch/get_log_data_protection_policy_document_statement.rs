#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLogDataProtectionPolicyDocumentStatement {
    /// Set of at least 1 sensitive data identifiers that you want to mask. Read more in [Types of data that you can protect](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/protect-sensitive-log-data-types.html).
    #[builder(into)]
    #[serde(rename = "dataIdentifiers")]
    pub r#data_identifiers: Vec<String>,
    /// Configures the data protection operation applied by this statement.
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Box<super::super::types::cloudwatch::GetLogDataProtectionPolicyDocumentStatementOperation>,
    /// Name of this statement.
    #[builder(into)]
    #[serde(rename = "sid")]
    pub r#sid: Option<String>,
}
