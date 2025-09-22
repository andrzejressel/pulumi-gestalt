#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableGlobalSecondaryIndex {
    /// Name of the hash key in the index; must be defined as an attribute in the resource.
    #[builder(into)]
    #[serde(rename = "hashKey")]
    pub r#hash_key: String,
    /// Name of the index.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Only required with `INCLUDE` as a projection type; a list of attributes to project into the index. These do not need to be defined as attributes on the table.
    #[builder(into)]
    #[serde(rename = "nonKeyAttributes")]
    pub r#non_key_attributes: Option<Vec<String>>,
    /// Sets the maximum number of read and write units for the specified on-demand table. See below.
    #[builder(into)]
    #[serde(rename = "onDemandThroughput")]
    pub r#on_demand_throughput: Option<Box<super::super::types::dynamodb::TableGlobalSecondaryIndexOnDemandThroughput>>,
    /// One of `ALL`, `INCLUDE` or `KEYS_ONLY` where `ALL` projects every attribute into the index, `KEYS_ONLY` projects  into the index only the table and index hash_key and sort_key attributes ,  `INCLUDE` projects into the index all of the attributes that are defined in `non_key_attributes` in addition to the attributes that that`KEYS_ONLY` project.
    #[builder(into)]
    #[serde(rename = "projectionType")]
    pub r#projection_type: String,
    /// Name of the range key; must be defined
    #[builder(into)]
    #[serde(rename = "rangeKey")]
    pub r#range_key: Option<String>,
    /// Number of read units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into)]
    #[serde(rename = "readCapacity")]
    pub r#read_capacity: Option<i32>,
    /// Number of write units for this index. Must be set if billing_mode is set to PROVISIONED.
    #[builder(into)]
    #[serde(rename = "writeCapacity")]
    pub r#write_capacity: Option<i32>,
}
