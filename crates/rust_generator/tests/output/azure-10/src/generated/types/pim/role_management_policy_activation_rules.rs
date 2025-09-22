#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleManagementPolicyActivationRules {
    /// An `approval_stage` block as defined below.
    #[builder(into)]
    #[serde(rename = "approvalStage")]
    pub r#approval_stage: Option<Box<super::super::types::pim::RoleManagementPolicyActivationRulesApprovalStage>>,
    /// The maximum length of time an activated role can be valid, in an ISO8601 Duration format (e.g. `PT8H`). Valid range is `PT30M` to `PT23H30M`, in 30 minute increments, or `PT1D`.
    #[builder(into)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Option<String>,
    /// Is approval required for activation. If `true` an `approval_stage` block must be provided.
    #[builder(into)]
    #[serde(rename = "requireApproval")]
    pub r#require_approval: Option<bool>,
    /// Is a justification required during activation of the role.
    #[builder(into)]
    #[serde(rename = "requireJustification")]
    pub r#require_justification: Option<bool>,
    /// Is multi-factor authentication required to activate the role. Conflicts with `required_conditional_access_authentication_context`.
    #[builder(into)]
    #[serde(rename = "requireMultifactorAuthentication")]
    pub r#require_multifactor_authentication: Option<bool>,
    /// Is ticket information requrired during activation of the role.
    #[builder(into)]
    #[serde(rename = "requireTicketInfo")]
    pub r#require_ticket_info: Option<bool>,
    /// The Entra ID Conditional Access context that must be present for activation. Conflicts with `require_multifactor_authentication`.
    #[builder(into)]
    #[serde(rename = "requiredConditionalAccessAuthenticationContext")]
    pub r#required_conditional_access_authentication_context: Option<String>,
}
