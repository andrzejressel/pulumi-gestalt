#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailSensitiveInformationPolicyConfig {
    /// List of entities. See PII Entities Config for more information.
    #[builder(into)]
    #[serde(rename = "piiEntitiesConfigs")]
    pub r#pii_entities_configs: Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigPiiEntitiesConfig>>,
    /// List of regex. See Regexes Config for more information.
    #[builder(into)]
    #[serde(rename = "regexesConfigs")]
    pub r#regexes_configs: Option<Vec<super::super::types::bedrock::GuardrailSensitiveInformationPolicyConfigRegexesConfig>>,
}
