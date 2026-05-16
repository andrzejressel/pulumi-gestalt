#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobDefinitionEksPropertiesPodPropertiesVolumeSecret {
    /// Whether the secret or the secret's keys must be defined.
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: Option<bool>,
    /// Name of the secret. The name must be allowed as a DNS subdomain name.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: String,
}
