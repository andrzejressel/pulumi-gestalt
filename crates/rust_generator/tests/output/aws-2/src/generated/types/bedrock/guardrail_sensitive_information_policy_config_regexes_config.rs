#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailSensitiveInformationPolicyConfigRegexesConfig {
    /// Options for sensitive information action.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The regex description.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The regex name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The regex pattern.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<String>,
}
