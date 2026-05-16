#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PrivateConnectionError {
    /// A list of messages that carry the error details.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Option<std::collections::HashMap<String, String>>,
    /// A message containing more information about the error that occurred.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}
