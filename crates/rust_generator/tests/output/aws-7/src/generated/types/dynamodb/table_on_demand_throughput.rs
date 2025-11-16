#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableOnDemandThroughput {
    /// Maximum number of read request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into)]
    #[serde(rename = "maxReadRequestUnits")]
    pub r#max_read_request_units: Option<i32>,
    /// Maximum number of write request units for the specified table. To specify set the value greater than or equal to 1. To remove set the value to -1.
    #[builder(into)]
    #[serde(rename = "maxWriteRequestUnits")]
    pub r#max_write_request_units: Option<i32>,
}
