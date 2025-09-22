#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceStatusCondition {
    /// Human readable message indicating details about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: String,
    /// One-word CamelCase reason for the condition's current status.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: String,
    /// Status of the condition, one of True, False, Unknown.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// Type of domain mapping condition.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
