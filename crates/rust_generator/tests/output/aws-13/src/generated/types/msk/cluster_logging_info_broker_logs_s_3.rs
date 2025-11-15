#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterLoggingInfoBrokerLogsS3 {
    /// Name of the S3 bucket to deliver logs to.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Prefix to append to the folder name.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
