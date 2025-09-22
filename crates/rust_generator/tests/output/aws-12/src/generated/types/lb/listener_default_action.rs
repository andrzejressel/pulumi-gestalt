#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerDefaultAction {
    /// Configuration block for using Amazon Cognito to authenticate users. Specify only when `type` is `authenticate-cognito`. See below.
    #[builder(into)]
    #[serde(rename = "authenticateCognito")]
    pub r#authenticate_cognito: Option<Box<super::super::types::lb::ListenerDefaultActionAuthenticateCognito>>,
    /// Configuration block for an identity provider that is compliant with OpenID Connect (OIDC). Specify only when `type` is `authenticate-oidc`. See below.
    #[builder(into)]
    #[serde(rename = "authenticateOidc")]
    pub r#authenticate_oidc: Option<Box<super::super::types::lb::ListenerDefaultActionAuthenticateOidc>>,
    /// Information for creating an action that returns a custom HTTP response. Required if `type` is `fixed-response`.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<Box<super::super::types::lb::ListenerDefaultActionFixedResponse>>,
    /// Configuration block for creating an action that distributes requests among one or more target groups. Specify only if `type` is `forward`. See below.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Option<Box<super::super::types::lb::ListenerDefaultActionForward>>,
    /// Order for the action. The action with the lowest value for order is performed first. Valid values are between `1` and `50000`. Defaults to the position in the list of actions.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// Configuration block for creating a redirect action. Required if `type` is `redirect`. See below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Option<Box<super::super::types::lb::ListenerDefaultActionRedirect>>,
    /// ARN of the Target Group to which to route traffic. Specify only if `type` is `forward` and you want to route to a single target group. To route to one or more target groups, use a `forward` block instead. Can be specified with `forward` but ARNs must match.
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Option<String>,
    /// Type of routing action. Valid values are `forward`, `redirect`, `fixed-response`, `authenticate-cognito` and `authenticate-oidc`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
