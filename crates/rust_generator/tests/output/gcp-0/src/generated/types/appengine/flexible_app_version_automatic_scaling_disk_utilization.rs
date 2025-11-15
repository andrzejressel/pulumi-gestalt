#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleAppVersionAutomaticScalingDiskUtilization {
    /// Target bytes read per second.
    #[builder(into)]
    #[serde(rename = "targetReadBytesPerSecond")]
    pub r#target_read_bytes_per_second: Option<i32>,
    /// Target ops read per seconds.
    #[builder(into)]
    #[serde(rename = "targetReadOpsPerSecond")]
    pub r#target_read_ops_per_second: Option<i32>,
    /// Target bytes written per second.
    #[builder(into)]
    #[serde(rename = "targetWriteBytesPerSecond")]
    pub r#target_write_bytes_per_second: Option<i32>,
    /// Target ops written per second.
    #[builder(into)]
    #[serde(rename = "targetWriteOpsPerSecond")]
    pub r#target_write_ops_per_second: Option<i32>,
}
