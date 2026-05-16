#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupPolicyAssignmentOverride {
    /// One or more `override_selector` block as defined below.
    #[builder(into)]
    #[serde(rename = "selectors")]
    pub r#selectors: Option<Vec<super::super::types::management::GroupPolicyAssignmentOverrideSelector>>,
    /// Specifies the value to override the policy property. Possible values for `policyEffect` override listed [policy effects](https://learn.microsoft.com/en-us/azure/governance/policy/concepts/effects).
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
