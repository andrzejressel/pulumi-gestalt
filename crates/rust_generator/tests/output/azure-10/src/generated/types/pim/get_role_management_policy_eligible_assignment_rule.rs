#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyEligibleAssignmentRule {
    /// (Boolean) Must an assignment have an expiry date.
    #[builder(into)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: bool,
    /// (String) The maximum length of time an assignment can be valid, as an ISO8601 duration.
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: String,
}
