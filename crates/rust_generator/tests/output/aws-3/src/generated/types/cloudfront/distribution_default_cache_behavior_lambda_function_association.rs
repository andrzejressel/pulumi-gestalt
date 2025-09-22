#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionDefaultCacheBehaviorLambdaFunctionAssociation {
    /// Specific event to trigger this function. Valid values: `viewer-request`, `origin-request`, `viewer-response`, `origin-response`.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: String,
    /// When set to true it exposes the request body to the lambda function. Defaults to false. Valid values: `true`, `false`.
    #[builder(into)]
    #[serde(rename = "includeBody")]
    pub r#include_body: Option<bool>,
    /// ARN of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: String,
}
