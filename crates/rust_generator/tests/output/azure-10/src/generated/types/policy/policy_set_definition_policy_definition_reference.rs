#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicySetDefinitionPolicyDefinitionReference {
    /// Parameter values for the referenced policy rule. This field is a JSON string that allows you to assign parameters to this policy rule.
    #[builder(into)]
    #[serde(rename = "parameterValues")]
    pub r#parameter_values: Option<String>,
    /// The ID of the policy definition that will be included in this policy set definition.
    #[builder(into)]
    #[serde(rename = "policyDefinitionId")]
    pub r#policy_definition_id: String,
    /// A list of names of the policy definition groups that this policy definition reference belongs to.
    #[builder(into)]
    #[serde(rename = "policyGroupNames")]
    pub r#policy_group_names: Option<Vec<String>>,
    /// A unique ID within this policy set definition for this policy definition reference.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
}
