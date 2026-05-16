#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateContainerEnvValueSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretKeyRef")]
    pub r#secret_key_ref: Option<Box<super::super::types::cloudrunv2::ServiceTemplateContainerEnvValueSourceSecretKeyRef>>,
}
