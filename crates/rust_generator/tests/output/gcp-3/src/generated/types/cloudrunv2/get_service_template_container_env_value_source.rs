#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTemplateContainerEnvValueSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    #[builder(into)]
    #[serde(rename = "secretKeyReves")]
    pub r#secret_key_reves: Vec<super::super::types::cloudrunv2::GetServiceTemplateContainerEnvValueSourceSecretKeyRef>,
}
