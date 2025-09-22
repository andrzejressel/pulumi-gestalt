#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetListenerRuleAction {
    /// An action to authenticate using Amazon Cognito.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticateCognito")]
    pub r#authenticate_cognito: Option<Box<super::super::types::lb::GetListenerRuleActionAuthenticateCognito>>,
    /// An action to authenticate using OIDC.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticateOidc")]
    pub r#authenticate_oidc: Option<Box<super::super::types::lb::GetListenerRuleActionAuthenticateOidc>>,
    /// An action to return a fixed response.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<Box<super::super::types::lb::GetListenerRuleActionFixedResponse>>,
    /// An action to forward the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Option<Box<super::super::types::lb::GetListenerRuleActionForward>>,
    /// The evaluation order of the action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: f64,
    /// An action to redirect the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Option<Box<super::super::types::lb::GetListenerRuleActionRedirect>>,
    /// The type of the action, indicates which sub-block will be populated.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
