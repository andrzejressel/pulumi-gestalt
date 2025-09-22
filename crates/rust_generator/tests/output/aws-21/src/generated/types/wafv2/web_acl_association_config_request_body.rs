#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclAssociationConfigRequestBody {
    /// Customizes the request body that your protected Amazon API Gateway REST APIs forward to AWS WAF for inspection. Applicable only when `scope` is set to `CLOUDFRONT`. See `api_gateway` below for details.
    #[builder(into)]
    #[serde(rename = "apiGateways")]
    pub r#api_gateways: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyApiGateway>>,
    /// Customizes the request body that your protected Amazon App Runner services forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `app_runner_service` below for details.
    #[builder(into)]
    #[serde(rename = "appRunnerServices")]
    pub r#app_runner_services: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyAppRunnerService>>,
    /// Customizes the request body that your protected Amazon CloudFront distributions forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cloudfront` below for details.
    #[builder(into)]
    #[serde(rename = "cloudfronts")]
    pub r#cloudfronts: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCloudfront>>,
    /// Customizes the request body that your protected Amazon Cognito user pools forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cognito_user_pool` below for details.
    #[builder(into)]
    #[serde(rename = "cognitoUserPools")]
    pub r#cognito_user_pools: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCognitoUserPool>>,
    /// Customizes the request body that your protected AWS Verfied Access instances forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `verified_access_instance` below for details.
    #[builder(into)]
    #[serde(rename = "verifiedAccessInstances")]
    pub r#verified_access_instances: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyVerifiedAccessInstance>>,
}
