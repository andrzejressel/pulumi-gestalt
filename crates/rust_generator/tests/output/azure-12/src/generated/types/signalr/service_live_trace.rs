#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceLiveTrace {
    /// Whether the log category `ConnectivityLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "connectivityLogsEnabled")]
    pub r#connectivity_logs_enabled: Option<bool>,
    /// Whether the live trace is enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Whether the log category `HttpRequestLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "httpRequestLogsEnabled")]
    pub r#http_request_logs_enabled: Option<bool>,
    /// Whether the log category `MessagingLogs` is enabled? Defaults to `true`
    #[builder(into)]
    #[serde(rename = "messagingLogsEnabled")]
    pub r#messaging_logs_enabled: Option<bool>,
}
