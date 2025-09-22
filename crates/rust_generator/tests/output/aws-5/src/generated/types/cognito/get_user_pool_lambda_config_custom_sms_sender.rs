#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetUserPoolLambdaConfigCustomSmsSender {
    /// - ARN of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: String,
    /// - Version of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaVersion")]
    pub r#lambda_version: String,
}
