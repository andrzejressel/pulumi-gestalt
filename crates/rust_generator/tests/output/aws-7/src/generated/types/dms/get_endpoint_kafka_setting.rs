#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointKafkaSetting {
    #[builder(into)]
    #[serde(rename = "broker")]
    pub r#broker: String,
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
    #[serde(rename = "messageMaxBytes")]
    pub r#message_max_bytes: i32,
    #[builder(into)]
    #[serde(rename = "noHexPrefix")]
    pub r#no_hex_prefix: bool,
    #[builder(into)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: bool,
    #[builder(into)]
    #[serde(rename = "saslPassword")]
    pub r#sasl_password: String,
    #[builder(into)]
    #[serde(rename = "saslUsername")]
    pub r#sasl_username: String,
    #[builder(into)]
    #[serde(rename = "securityProtocol")]
    pub r#security_protocol: String,
    #[builder(into)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientCertificateArn")]
    pub r#ssl_client_certificate_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientKeyArn")]
    pub r#ssl_client_key_arn: String,
    #[builder(into)]
    #[serde(rename = "sslClientKeyPassword")]
    pub r#ssl_client_key_password: String,
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}
