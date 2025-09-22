#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscaleSettingNotificationEmail {
    /// Specifies a list of custom email addresses to which the email notifications will be sent.
    #[builder(into)]
    #[serde(rename = "customEmails")]
    pub r#custom_emails: Option<Vec<String>>,
    /// Should email notifications be sent to the subscription administrator? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendToSubscriptionAdministrator")]
    pub r#send_to_subscription_administrator: Option<bool>,
    /// Should email notifications be sent to the subscription co-administrator? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sendToSubscriptionCoAdministrator")]
    pub r#send_to_subscription_co_administrator: Option<bool>,
}
