#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SlotLogs {
    /// An `application_logs` block as defined below.
    #[builder(into)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Option<Box<super::super::types::appservice::SlotLogsApplicationLogs>>,
    /// Should `Detailed error messages` be enabled on this App Service slot? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "detailedErrorMessagesEnabled")]
    pub r#detailed_error_messages_enabled: Option<bool>,
    /// Should `Failed request tracing` be enabled on this App Service slot? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "failedRequestTracingEnabled")]
    pub r#failed_request_tracing_enabled: Option<bool>,
    /// An `http_logs` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Option<Box<super::super::types::appservice::SlotLogsHttpLogs>>,
}
