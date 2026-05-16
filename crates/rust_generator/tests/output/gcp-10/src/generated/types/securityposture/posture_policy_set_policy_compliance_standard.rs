#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PosturePolicySetPolicyComplianceStandard {
    /// Mapping of security controls for the policy.
    #[builder(into)]
    #[serde(rename = "control")]
    pub r#control: Option<String>,
    /// Mapping of compliance standards for the policy.
    #[builder(into)]
    #[serde(rename = "standard")]
    pub r#standard: Option<String>,
}
