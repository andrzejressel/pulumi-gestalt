#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailTopicPolicyConfigTopicsConfig {
    /// Definition of topic in topic policy.
    #[builder(into)]
    #[serde(rename = "definition")]
    pub r#definition: String,
    /// List of text examples.
    #[builder(into)]
    #[serde(rename = "examples")]
    pub r#examples: Option<Vec<String>>,
    /// Name of topic in topic policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Type of topic in a policy.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
