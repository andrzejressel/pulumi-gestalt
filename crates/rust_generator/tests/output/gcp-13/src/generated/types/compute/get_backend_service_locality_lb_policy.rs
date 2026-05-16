#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBackendServiceLocalityLbPolicy {
    /// The configuration for a custom policy implemented by the user and
    /// deployed with the client.
    #[builder(into)]
    #[serde(rename = "customPolicies")]
    pub r#custom_policies: Vec<super::super::types::compute::GetBackendServiceLocalityLbPolicyCustomPolicy>,
    /// The configuration for a built-in load balancing policy.
    #[builder(into)]
    #[serde(rename = "policies")]
    pub r#policies: Vec<super::super::types::compute::GetBackendServiceLocalityLbPolicyPolicy>,
}
