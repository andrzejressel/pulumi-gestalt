#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppSlotLogs {
    /// A `application_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationLogs")]
    pub r#application_logs: Box<Option<super::super::types::appservice::LinuxWebAppSlotLogsApplicationLogs>>,
    /// Should detailed error messages be enabled?
    #[builder(into)]
    #[serde(rename = "detailedErrorMessages")]
    pub r#detailed_error_messages: Option<bool>,
    /// Should the failed request tracing be enabled?
    #[builder(into)]
    #[serde(rename = "failedRequestTracing")]
    pub r#failed_request_tracing: Option<bool>,
    /// An `http_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "httpLogs")]
    pub r#http_logs: Box<Option<super::super::types::appservice::LinuxWebAppSlotLogsHttpLogs>>,
}
