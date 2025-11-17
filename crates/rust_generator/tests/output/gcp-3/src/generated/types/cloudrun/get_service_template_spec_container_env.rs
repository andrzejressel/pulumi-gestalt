#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTemplateSpecContainerEnv {
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Defaults to "".
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
    /// Source for the environment variable's value. Only supports secret_key_ref.
    #[builder(into)]
    #[serde(rename = "valueFroms")]
    pub r#value_froms: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvValueFrom>,
}
