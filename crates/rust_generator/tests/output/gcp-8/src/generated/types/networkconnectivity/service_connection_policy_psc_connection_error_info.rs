#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceConnectionPolicyPscConnectionErrorInfo {
    /// The logical grouping to which the "reason" belongs.
    #[builder(into)]
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
    /// Additional structured details about this error.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Option<std::collections::HashMap<String, String>>,
    /// The reason of the error.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}
