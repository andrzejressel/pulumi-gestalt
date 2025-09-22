#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLoggingV2TargetObjectKeyFormatPartitionedPrefix {
    /// Specifies the partition date source for the partitioned prefix. Valid values: `EventTime`, `DeliveryTime`.
    #[builder(into)]
    #[serde(rename = "partitionDateSource")]
    pub r#partition_date_source: String,
}
