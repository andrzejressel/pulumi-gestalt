#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleAction {
    /// Information for creating an authenticate action using Cognito. Required if `type` is `authenticate-cognito`.
    #[builder(into)]
    #[serde(rename = "authenticateCognito")]
    pub r#authenticate_cognito: Box<Option<super::super::types::lb::ListenerRuleActionAuthenticateCognito>>,
    /// Information for creating an authenticate action using OIDC. Required if `type` is `authenticate-oidc`.
    #[builder(into)]
    #[serde(rename = "authenticateOidc")]
    pub r#authenticate_oidc: Box<Option<super::super::types::lb::ListenerRuleActionAuthenticateOidc>>,
    /// Information for creating an action that returns a custom HTTP response. Required if `type` is `fixed-response`.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<super::super::types::lb::ListenerRuleActionFixedResponse>>,
    /// Configuration block for creating an action that distributes requests among one or more target groups.
    /// Specify only if `type` is `forward`.
    /// Cannot be specified with `target_group_arn`.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Box<Option<super::super::types::lb::ListenerRuleActionForward>>,
    /// Order for the action.
    /// The action with the lowest value for order is performed first.
    /// Valid values are between `1` and `50000`.
    /// Defaults to the position in the list of actions.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: Option<i32>,
    /// Information for creating a redirect action. Required if `type` is `redirect`.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Box<Option<super::super::types::lb::ListenerRuleActionRedirect>>,
    /// ARN of the Target Group to which to route traffic.
    /// Specify only if `type` is `forward` and you want to route to a single target group.
    /// To route to one or more target groups, use a `forward` block instead.
    /// Cannot be specified with `forward`.
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Option<String>,
    /// The type of routing action. Valid values are `forward`, `redirect`, `fixed-response`, `authenticate-cognito` and `authenticate-oidc`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
