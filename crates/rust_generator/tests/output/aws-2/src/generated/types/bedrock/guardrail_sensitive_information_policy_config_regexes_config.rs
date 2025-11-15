#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuardrailSensitiveInformationPolicyConfigRegexesConfig {
    /// Options for sensitive information action.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The regex description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The regex name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The regex pattern.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: String,
}
