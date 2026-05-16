#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyDefinition {
    /// The static policy statement. See Static below.
    #[builder(into)]
    #[serde(rename = "static")]
    pub r#static_: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionStatic>>,
    /// The template linked policy. See Template Linked below.
    #[builder(into)]
    #[serde(rename = "templateLinked")]
    pub r#template_linked: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinked>>,
}
