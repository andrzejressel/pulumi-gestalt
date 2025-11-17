#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTemplateSpecContainerEnvFromSecretRef {
    /// The Secret to select from.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "localObjectReference")]
    pub r#local_object_reference: Option<Box<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromSecretRefLocalObjectReference>>,
    /// Specify whether the Secret must be defined
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: Option<bool>,
}
