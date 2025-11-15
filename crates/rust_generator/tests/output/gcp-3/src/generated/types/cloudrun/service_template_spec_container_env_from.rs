#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateSpecContainerEnvFrom {
    /// The ConfigMap to select from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "configMapRef")]
    pub r#config_map_ref: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromConfigMapRef>>,
    /// An optional identifier to prepend to each key in the ConfigMap.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// The Secret to select from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretRef")]
    pub r#secret_ref: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromSecretRef>>,
}
