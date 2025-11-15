#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBudgetSubscriptionNotification {
    /// A list of email addresses to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactEmails")]
    pub r#contact_emails: Vec<String>,
    /// A list of Action Group IDs to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactGroups")]
    pub r#contact_groups: Vec<String>,
    /// A list of contact roles to send the budget notification to when the threshold is exceeded.
    #[builder(into)]
    #[serde(rename = "contactRoles")]
    pub r#contact_roles: Vec<String>,
    /// Whether the notification is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// The operator to used for comparison.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Threshold value associated with the notification.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: i32,
    #[builder(into)]
    #[serde(rename = "thresholdType")]
    pub r#threshold_type: String,
}
