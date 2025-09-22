#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerEnv {
    /// Name of the environment variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Defaults to "".
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// Source for the environment variable's value. Only supports secret_key_ref.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "valueFrom")]
    pub r#value_from: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvValueFrom>>,
}
