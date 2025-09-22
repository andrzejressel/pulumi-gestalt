#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct QueueAppEngineRoutingOverride {
    /// (Output)
    /// The host that the task is sent to.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// App instance.
    /// By default, the task is sent to an instance which is available when the task is attempted.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: Option<String>,
    /// App service.
    /// By default, the task is sent to the service which is the default service when the task is attempted.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// App version.
    /// By default, the task is sent to the version which is the default version when the task is attempted.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
