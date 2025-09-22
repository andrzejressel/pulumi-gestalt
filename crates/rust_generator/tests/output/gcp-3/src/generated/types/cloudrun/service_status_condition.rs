#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceStatusCondition {
    /// (Output)
    /// Human readable message indicating details about the current status.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// (Output)
    /// One-word CamelCase reason for the condition's current status.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
    /// (Output)
    /// Status of the condition, one of True, False, Unknown.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// (Output)
    /// Type of domain mapping condition.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
