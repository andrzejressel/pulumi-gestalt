#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HttpRouteRuleAction {
    /// The specification for allowing client side cross-origin requests.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Box<Option<super::super::types::networkservices::HttpRouteRuleActionCorsPolicy>>,
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkservices::HttpRouteRuleActionDestination>>,
    /// The specification for fault injection introduced into traffic to test the resiliency of clients to backend service failure.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Box<Option<super::super::types::networkservices::HttpRouteRuleActionFaultInjectionPolicy>>,
    /// If set, the request is directed as configured by this field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Box<Option<super::super::types::networkservices::HttpRouteRuleActionRedirect>>,
    /// The specification for modifying the headers of a matching request prior to delivery of the request to the destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeaderModifier")]
    pub r#request_header_modifier: Box<Option<super::super::types::networkservices::HttpRouteRuleActionRequestHeaderModifier>>,
    /// Specifies the policy on how requests intended for the routes destination are shadowed to a separate mirrored destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestMirrorPolicy")]
    pub r#request_mirror_policy: Box<Option<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicy>>,
    /// The specification for modifying the headers of a response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeaderModifier")]
    pub r#response_header_modifier: Box<Option<super::super::types::networkservices::HttpRouteRuleActionResponseHeaderModifier>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::networkservices::HttpRouteRuleActionRetryPolicy>>,
    /// Specifies the timeout for selected route.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
    /// The specification for rewrite URL before forwarding requests to the destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Box<Option<super::super::types::networkservices::HttpRouteRuleActionUrlRewrite>>,
}
