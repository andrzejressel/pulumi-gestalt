#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceTemplateSpecContainerEnvFrom {
    /// The ConfigMap to select from.
    #[builder(into)]
    #[serde(rename = "configMapReves")]
    pub r#config_map_reves: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFromConfigMapRef>,
    /// An optional identifier to prepend to each key in the ConfigMap.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    /// The Secret to select from.
    #[builder(into)]
    #[serde(rename = "secretReves")]
    pub r#secret_reves: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFromSecretRef>,
}
