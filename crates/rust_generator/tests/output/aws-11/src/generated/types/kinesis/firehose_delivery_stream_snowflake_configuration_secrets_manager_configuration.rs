#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamSnowflakeConfigurationSecretsManagerConfiguration {
    /// Enables or disables the Secrets Manager configuration.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// The ARN of the role the stream assumes.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
    /// The ARN of the Secrets Manager secret. This value is required if `enabled` is true.
    #[builder(into, default)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Box<Option<String>>,
}
