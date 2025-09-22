#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetActionGroupAutomationRunbookReceiver {
    /// The automation account ID which holds this runbook and authenticates to Azure resources.
    #[builder(into)]
    #[serde(rename = "automationAccountId")]
    pub r#automation_account_id: String,
    /// Indicates whether this instance is global runbook.
    #[builder(into)]
    #[serde(rename = "isGlobalRunbook")]
    pub r#is_global_runbook: bool,
    /// Specifies the name of the Action Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name for this runbook.
    #[builder(into)]
    #[serde(rename = "runbookName")]
    pub r#runbook_name: String,
    /// The URI where webhooks should be sent.
    #[builder(into)]
    #[serde(rename = "serviceUri")]
    pub r#service_uri: String,
    /// Indicates whether to use common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: bool,
    /// The resource id for webhook linked to this runbook.
    #[builder(into)]
    #[serde(rename = "webhookResourceId")]
    pub r#webhook_resource_id: String,
}
