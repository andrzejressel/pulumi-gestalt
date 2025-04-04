#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobTemplateTemplateContainerEnvValueSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    #[builder(into)]
    #[serde(rename = "secretKeyReves")]
    pub r#secret_key_reves: Box<Vec<super::super::types::cloudrunv2::GetJobTemplateTemplateContainerEnvValueSourceSecretKeyRef>>,
}
