#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerConnectionLogs {
    /// S3 bucket name to store the logs in.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Boolean to enable / disable `connection_logs`. Defaults to `false`, even when `bucket` is specified.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// S3 bucket prefix. Logs are stored in the root if not configured.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
