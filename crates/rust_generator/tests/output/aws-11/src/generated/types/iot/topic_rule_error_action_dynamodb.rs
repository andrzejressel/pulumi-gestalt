#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionDynamodb {
    /// The hash key name.
    #[builder(into)]
    #[serde(rename = "hashKeyField")]
    pub r#hash_key_field: String,
    /// The hash key type. Valid values are "STRING" or "NUMBER".
    #[builder(into)]
    #[serde(rename = "hashKeyType")]
    pub r#hash_key_type: Option<String>,
    /// The hash key value.
    #[builder(into)]
    #[serde(rename = "hashKeyValue")]
    pub r#hash_key_value: String,
    /// The operation. Valid values are "INSERT", "UPDATE", or "DELETE".
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    /// The action payload.
    #[builder(into)]
    #[serde(rename = "payloadField")]
    pub r#payload_field: Option<String>,
    /// The range key name.
    #[builder(into)]
    #[serde(rename = "rangeKeyField")]
    pub r#range_key_field: Option<String>,
    /// The range key type. Valid values are "STRING" or "NUMBER".
    #[builder(into)]
    #[serde(rename = "rangeKeyType")]
    pub r#range_key_type: Option<String>,
    /// The range key value.
    #[builder(into)]
    #[serde(rename = "rangeKeyValue")]
    pub r#range_key_value: Option<String>,
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
}
