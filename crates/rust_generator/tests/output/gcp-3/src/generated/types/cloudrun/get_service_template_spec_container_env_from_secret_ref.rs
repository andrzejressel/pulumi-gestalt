#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceTemplateSpecContainerEnvFromSecretRef {
    /// The Secret to select from.
    #[builder(into)]
    #[serde(rename = "localObjectReferences")]
    pub r#local_object_references: Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFromSecretRefLocalObjectReference>,
    /// Specify whether the Secret must be defined
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: bool,
}
