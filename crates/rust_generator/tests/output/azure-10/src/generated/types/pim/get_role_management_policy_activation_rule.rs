#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRoleManagementPolicyActivationRule {
    /// An `approval_stage` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvalStages")]
    pub r#approval_stages: Vec<super::super::types::pim::GetRoleManagementPolicyActivationRuleApprovalStage>,
    /// (String) The maximum length of time an activated role can be valid, in an ISO8601 Duration format.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: String,
    /// (Boolean) Is approval required for activation.
    #[builder(into)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: bool,
    /// (Boolean) Is a justification required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: bool,
    /// (Boolean) Is multi-factor authentication required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: bool,
    /// (Boolean) Is ticket information required to create new assignments.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: bool,
    /// (String) The Entra ID Conditional Access context that must be present for activation.
    #[builder(into)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: String,
}
