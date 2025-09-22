#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscaleSettingNotification {
    /// A `email` block as defined below.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<super::super::types::monitoring::AutoscaleSettingNotificationEmail>>,
    /// One or more `webhook` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "webhooks")]
    pub r#webhooks: Option<Vec<super::super::types::monitoring::AutoscaleSettingNotificationWebhook>>,
}
