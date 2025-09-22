#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ObjectLambdaAccessPointConfigurationTransformationConfigurationContentTransformationAwsLambda {
    /// The Amazon Resource Name (ARN) of the AWS Lambda function.
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: String,
    /// Additional JSON that provides supplemental data to the Lambda function used to transform objects.
    #[builder(into)]
    #[serde(rename = "functionPayload")]
    pub r#function_payload: Option<String>,
}
