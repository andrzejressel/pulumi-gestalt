#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamMskSourceConfigurationAuthenticationConfiguration {
    /// The type of connectivity used to access the Amazon MSK cluster. Valid values: `PUBLIC`, `PRIVATE`.
    #[builder(into)]
    #[serde(rename = "connectivity")]
    pub r#connectivity: Box<String>,
    /// The ARN of the role used to access the Amazon MSK cluster.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
