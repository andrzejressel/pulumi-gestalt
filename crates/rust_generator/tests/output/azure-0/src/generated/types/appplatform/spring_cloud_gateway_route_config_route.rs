#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudGatewayRouteConfigRoute {
    /// Specifies the classification tags which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "classificationTags")]
    pub r#classification_tags: Option<Vec<String>>,
    /// Specifies the description which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Specifies a list of filters which are used to modify the request before sending it to the target endpoint, or the received response.
    #[builder(into)]
    #[serde(rename = "filters")]
    pub r#filters: Option<Vec<String>>,
    /// Specifies the route processing order.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    /// Specifies a list of conditions to evaluate a route for each request. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request.
    #[builder(into)]
    #[serde(rename = "predicates")]
    pub r#predicates: Option<Vec<String>>,
    /// Should the sso validation be enabled?
    #[builder(into)]
    #[serde(rename = "ssoValidationEnabled")]
    pub r#sso_validation_enabled: Option<bool>,
    /// Specifies the title which will be applied to methods in the generated OpenAPI documentation.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Should pass currently-authenticated user's identity token to application service?
    #[builder(into)]
    #[serde(rename = "tokenRelay")]
    pub r#token_relay: Option<bool>,
    /// Specifies the full uri which will override `appName`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}
