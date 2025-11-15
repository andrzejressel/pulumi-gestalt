#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionResizeRequestStatusErrorErrorErrorDetailErrorInfo {
    /// (Output)
    /// The logical grouping to which the "reason" belongs. The error domain is typically the registered service name of the tool or product that generates the error. Example: "pubsub.googleapis.com".
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// (Output)
    /// Additional structured details about this error.
    #[builder(into)]
    #[serde(rename = "metadatas")]
    pub r#metadatas: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// The reason of the error. This is a constant value that identifies the proximate cause of the error. Error reasons are unique within a particular domain of errors.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
