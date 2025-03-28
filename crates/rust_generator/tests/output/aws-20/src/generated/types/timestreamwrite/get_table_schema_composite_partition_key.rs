#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableSchemaCompositePartitionKey {
    #[builder(into)]
    #[serde(rename = "enforcementInRecord")]
    pub r#enforcement_in_record: Box<String>,
    /// Name of the Timestream table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Type of partition key.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
