#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoleManagementPolicyNotificationRulesActiveAssignmentsAdminNotifications {
    /// The additional recipients to notify
    #[builder(into)]
    #[serde(rename = "additionalRecipients")]
    pub r#additional_recipients: Option<Vec<String>>,
    /// Whether the default recipients are notified
    #[builder(into)]
    #[serde(rename = "defaultRecipients")]
    pub r#default_recipients: bool,
    /// What level of notifications are sent
    #[builder(into)]
    #[serde(rename = "notificationLevel")]
    pub r#notification_level: String,
}
