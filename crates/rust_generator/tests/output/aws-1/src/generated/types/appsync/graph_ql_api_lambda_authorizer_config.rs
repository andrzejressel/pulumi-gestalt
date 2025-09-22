#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GraphQlApiLambdaAuthorizerConfig {
    /// Number of seconds a response should be cached for. The default is 5 minutes (300 seconds). The Lambda function can override this by returning a `ttlOverride` key in its response. A value of 0 disables caching of responses. Minimum value of 0. Maximum value of 3600.
    #[builder(into)]
    #[serde(rename = "authorizerResultTtlInSeconds")]
    pub r#authorizer_result_ttl_in_seconds: Option<i32>,
    /// ARN of the Lambda function to be called for authorization. Note: This Lambda function must have a resource-based policy assigned to it, to allow `lambda:InvokeFunction` from service principal `appsync.amazonaws.com`.
    #[builder(into)]
    #[serde(rename = "authorizerUri")]
    pub r#authorizer_uri: String,
    /// Regular expression for validation of tokens before the Lambda function is called.
    #[builder(into)]
    #[serde(rename = "identityValidationExpression")]
    pub r#identity_validation_expression: Option<String>,
}
