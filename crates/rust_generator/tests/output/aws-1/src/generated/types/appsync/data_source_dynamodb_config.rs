#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataSourceDynamodbConfig {
    /// The DeltaSyncConfig for a versioned data source. See `delta_sync_config` Block for details.
    #[builder(into)]
    #[serde(rename = "deltaSyncConfig")]
    pub r#delta_sync_config: Option<Box<super::super::types::appsync::DataSourceDynamodbConfigDeltaSyncConfig>>,
    /// AWS region of the DynamoDB table. Defaults to current region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// Set to `true` to use Amazon Cognito credentials with this data source.
    #[builder(into)]
    #[serde(rename = "useCallerCredentials")]
    pub r#use_caller_credentials: Option<bool>,
    /// Detects Conflict Detection and Resolution with this data source.
    #[builder(into)]
    #[serde(rename = "versioned")]
    pub r#versioned: Option<bool>,
}
