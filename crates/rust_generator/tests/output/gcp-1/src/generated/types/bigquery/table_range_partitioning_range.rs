#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableRangePartitioningRange {
    /// End of the range partitioning, exclusive.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: i32,
    /// The width of each range within the partition.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: i32,
    /// Start of the range partitioning, inclusive.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: i32,
}
