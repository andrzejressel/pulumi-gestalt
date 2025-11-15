#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReceivedLicenseReceivedMetadata {
    /// A list of allowed operations.
    #[builder(into)]
    #[serde(rename = "allowedOperations")]
    pub r#allowed_operations: Vec<String>,
    /// Received status.
    #[builder(into)]
    #[serde(rename = "receivedStatus")]
    pub r#received_status: String,
    /// Received status reason.
    #[builder(into)]
    #[serde(rename = "receivedStatusReason")]
    pub r#received_status_reason: String,
}
