#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobEventTriggerConfigScaleRuleAuthentication {
    /// Name of the secret from which to pull the auth params.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: String,
    /// Trigger Parameter that uses the secret.
    #[builder(into)]
    #[serde(rename = "triggerParameter")]
    pub r#trigger_parameter: String,
}
