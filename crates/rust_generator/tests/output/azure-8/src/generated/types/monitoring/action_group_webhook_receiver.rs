#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupWebhookReceiver {
    /// The `aad_auth` block as defined below.
    /// 
    /// > **NOTE:** Before adding a secure webhook receiver by setting `aad_auth`, please read [the configuration instruction of the AAD application](https://docs.microsoft.com/azure/azure-monitor/platform/action-groups#secure-webhook).
    #[builder(into)]
    #[serde(rename = "aadAuth")]
    pub r#aad_auth: Option<Box<super::super::types::monitoring::ActionGroupWebhookReceiverAadAuth>>,
    /// The name of the webhook receiver. Names must be unique (case-insensitive) across all receivers within an action group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The URI where webhooks should be sent.
    #[builder(into)]
    #[serde(rename = "serviceUri")]
    pub r#service_uri: String,
    /// Enables or disables the common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Option<bool>,
}
