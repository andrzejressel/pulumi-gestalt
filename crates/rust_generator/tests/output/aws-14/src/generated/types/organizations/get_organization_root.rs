#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOrganizationRoot {
    /// ARN of the root
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Identifier of the root
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The name of the policy type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// List of policy types enabled for this root. All elements have these attributes:
    #[builder(into)]
    #[serde(rename = "policyTypes")]
    pub r#policy_types: Vec<super::super::types::organizations::GetOrganizationRootPolicyType>,
}
