#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerAccessLogs {
    /// The S3 bucket name to store the logs in.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// The S3 bucket prefix. Logs are stored in the root if not configured.
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Option<String>,
    /// Boolean to enable / disable `access_logs`. Default is `true`
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The publishing interval in minutes. Valid values: `5` and `60`. Default: `60`
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Option<i32>,
}
