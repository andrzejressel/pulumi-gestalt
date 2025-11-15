#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupAzureFunctionReceiver {
    /// The Azure resource ID of the function app.
    #[builder(into)]
    #[serde(rename = "functionAppResourceId")]
    pub r#function_app_resource_id: String,
    /// The function name in the function app.
    #[builder(into)]
    #[serde(rename = "functionName")]
    pub r#function_name: String,
    /// The HTTP trigger url where HTTP request sent to.
    #[builder(into)]
    #[serde(rename = "httpTriggerUrl")]
    pub r#http_trigger_url: String,
    /// The name of the Azure Function receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Enables or disables the common alert schema.
    #[builder(into)]
    #[serde(rename = "useCommonAlertSchema")]
    pub r#use_common_alert_schema: Option<bool>,
}
