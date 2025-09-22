#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QuicksetupConfigurationManagerStatusSummary {
    /// Current status.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// When applicable, returns an informational message relevant to the current status and status type of the status summary object.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: String,
    /// Type of a status summary.
    #[builder(into)]
    #[serde(rename = "statusType")]
    pub r#status_type: String,
}
