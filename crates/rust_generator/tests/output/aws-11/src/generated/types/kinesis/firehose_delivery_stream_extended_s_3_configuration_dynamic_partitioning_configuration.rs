#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDynamicPartitioningConfiguration {
    /// Enables or disables dynamic partitioning. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Total amount of seconds Firehose spends on retries. Valid values between 0 and 7200. Default is 300.
    /// 
    /// > **NOTE:** You can enable dynamic partitioning only when you create a new delivery stream. Once you enable dynamic partitioning on a delivery stream, it cannot be disabled on this delivery stream. Therefore, the provider will recreate the resource whenever dynamic partitioning is enabled or disabled.
    #[builder(into)]
    #[serde(rename = "retryDuration")]
    pub r#retry_duration: Option<i32>,
}
