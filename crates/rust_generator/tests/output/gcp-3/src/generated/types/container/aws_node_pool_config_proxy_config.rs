#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AwsNodePoolConfigProxyConfig {
    /// The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: String,
    /// The version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: String,
}
