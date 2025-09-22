#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerEnvFromConfigMapRef {
    /// The ConfigMap to select from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "localObjectReference")]
    pub r#local_object_reference: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromConfigMapRefLocalObjectReference>>,
    /// Specify whether the ConfigMap must be defined
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: Option<bool>,
}
