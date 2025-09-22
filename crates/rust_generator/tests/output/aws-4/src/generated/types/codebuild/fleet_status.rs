#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetStatus {
    /// Additional information about a compute fleet.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<String>,
    /// Message associated with the status of a compute fleet.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Status code of the compute fleet.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<String>,
}
