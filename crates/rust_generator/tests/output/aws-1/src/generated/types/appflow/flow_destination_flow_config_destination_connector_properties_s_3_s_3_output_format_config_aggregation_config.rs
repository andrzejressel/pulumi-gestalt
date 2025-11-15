#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig {
    /// Whether Amazon AppFlow aggregates the flow records into a single file, or leave them unaggregated. Valid values are `None` and `SingleFile`.
    #[builder(into)]
    #[serde(rename = "aggregationType")]
    pub r#aggregation_type: Option<String>,
    /// The desired file size, in MB, for each output file that Amazon AppFlow writes to the flow destination. Integer value.
    #[builder(into)]
    #[serde(rename = "targetFileSize")]
    pub r#target_file_size: Option<i32>,
}
