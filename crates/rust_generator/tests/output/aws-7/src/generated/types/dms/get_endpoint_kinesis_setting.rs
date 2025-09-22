#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointKinesisSetting {
    #[builder(into)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: bool,
    #[builder(into)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: bool,
    #[builder(into)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: bool,
    #[builder(into)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: bool,
    #[builder(into)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: bool,
    #[builder(into)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: String,
    #[builder(into)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: bool,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: String,
}
