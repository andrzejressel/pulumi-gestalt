#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleManagementPolicyNotificationRules {
    /// A `notification_target` block as defined below to configure notfications on active role assignments.
    #[builder(into)]
    #[serde(rename = "activeAssignments")]
    pub r#active_assignments: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesActiveAssignments>>,
    /// A `notification_target` block as defined below for configuring notifications on activation of eligible role.
    #[builder(into)]
    #[serde(rename = "eligibleActivations")]
    pub r#eligible_activations: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivations>>,
    /// A `notification_target` block as defined below to configure notification on eligible role assignments.
    /// 
    /// At least one `notification_target` block must be provided.
    #[builder(into)]
    #[serde(rename = "eligibleAssignments")]
    pub r#eligible_assignments: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleAssignments>>,
}
