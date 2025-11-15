#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyNotificationRulesEligibleActivations {
    /// Admin notification settings
    #[builder(into)]
    #[serde(rename = "adminNotifications")]
    pub r#admin_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsAdminNotifications>>,
    /// Approver notification settings
    #[builder(into)]
    #[serde(rename = "approverNotifications")]
    pub r#approver_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsApproverNotifications>>,
    /// Assignee notification settings
    #[builder(into)]
    #[serde(rename = "assigneeNotifications")]
    pub r#assignee_notifications: Option<Box<super::super::types::pim::RoleManagementPolicyNotificationRulesEligibleActivationsAssigneeNotifications>>,
}
