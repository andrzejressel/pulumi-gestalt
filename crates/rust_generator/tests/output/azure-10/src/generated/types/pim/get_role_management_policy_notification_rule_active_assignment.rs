#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoleManagementPolicyNotificationRuleActiveAssignment {
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "adminNotifications")]
    pub r#admin_notifications: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleActiveAssignmentAdminNotification>,
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "approverNotifications")]
    pub r#approver_notifications: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleActiveAssignmentApproverNotification>,
    /// A `notification_settings` block as defined above.
    #[builder(into)]
    #[serde(rename = "assigneeNotifications")]
    pub r#assignee_notifications: Vec<super::super::types::pim::GetRoleManagementPolicyNotificationRuleActiveAssignmentAssigneeNotification>,
}
