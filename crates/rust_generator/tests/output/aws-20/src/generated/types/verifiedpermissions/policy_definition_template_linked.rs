#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyDefinitionTemplateLinked {
    /// The ID of the template.
    #[builder(into)]
    #[serde(rename = "policyTemplateId")]
    pub r#policy_template_id: String,
    /// The principal of the template linked policy.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedPrincipal>>,
    /// The resource of the template linked policy.
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Option<Box<super::super::types::verifiedpermissions::PolicyDefinitionTemplateLinkedResource>>,
}
