#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledQueryRulesAlertAction {
    /// List of action group reference resource IDs.
    #[builder(into)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Vec<String>,
    /// Custom payload to be sent for all webhook payloads in alerting action.
    #[builder(into)]
    #[serde(rename = "customWebhookPayload")]
    pub r#custom_webhook_payload: Option<String>,
    /// Custom subject override for all email ids in Azure action group.
    #[builder(into)]
    #[serde(rename = "emailSubject")]
    pub r#email_subject: Option<String>,
}
