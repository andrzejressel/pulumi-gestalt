#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleManagementPolicyEligibleAssignmentRules {
    /// Must an assignment have an expiry date. `false` allows permanent assignment.
    #[builder(into)]
    #[serde(rename = "expirationRequired")]
    pub r#expiration_required: Option<bool>,
    /// The maximum length of time an assignment can be valid, as an ISO8601 duration. Permitted values: `P15D`, `P30D`, `P90D`, `P180D`, or `P365D`.
    /// 
    /// One of `expiration_required` or `expire_after` must be provided.
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Option<String>,
}
