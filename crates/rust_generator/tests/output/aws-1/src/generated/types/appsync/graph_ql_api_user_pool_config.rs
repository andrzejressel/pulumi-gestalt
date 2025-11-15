#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiUserPoolConfig {
    /// Regular expression for validating the incoming Amazon Cognito User Pool app client ID.
    #[builder(into)]
    #[serde(rename = "appIdClientRegex")]
    pub r#app_id_client_regex: Option<String>,
    /// AWS region in which the user pool was created.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Option<String>,
    /// Action that you want your GraphQL API to take when a request that uses Amazon Cognito User Pool authentication doesn't match the Amazon Cognito User Pool configuration. Valid: `ALLOW` and `DENY`
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: String,
    /// User pool ID.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: String,
}
