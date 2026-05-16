#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowLogDestinationOptions {
    /// The format for the flow log. Default value: `plain-text`. Valid values: `plain-text`, `parquet`.
    #[builder(into)]
    #[serde(rename = "fileFormat")]
    pub r#file_format: Option<String>,
    /// Indicates whether to use Hive-compatible prefixes for flow logs stored in Amazon S3. Default value: `false`.
    #[builder(into)]
    #[serde(rename = "hiveCompatiblePartitions")]
    pub r#hive_compatible_partitions: Option<bool>,
    /// Indicates whether to partition the flow log per hour. This reduces the cost and response time for queries. Default value: `false`.
    #[builder(into)]
    #[serde(rename = "perHourPartition")]
    pub r#per_hour_partition: Option<bool>,
}
