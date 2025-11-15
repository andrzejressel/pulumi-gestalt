#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SmartDetectorAlertRuleActionGroup {
    /// Specifies a custom email subject if Email Receiver is specified in Monitor Action Group resource.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Option<String>,
    /// Specifies the action group ids.
    #[builder(into)]
    #[serde(rename = "ids")]
    pub r#ids: Vec<String>,
    /// A JSON String which Specifies the custom webhook payload if Webhook Receiver is specified in Monitor Action Group resource.
    #[builder(into)]
    #[serde(rename = "webhookPayload")]
    pub r#webhook_payload: Option<String>,
}
