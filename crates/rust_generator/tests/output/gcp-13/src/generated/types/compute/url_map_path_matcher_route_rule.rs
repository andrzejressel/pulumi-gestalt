#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapPathMatcherRouteRule {
    /// Specifies changes to request and response headers that need to take effect for
    /// the selected backendService. The headerAction specified here are applied before
    /// the matching pathMatchers[].headerAction and after pathMatchers[].routeRules[].r
    /// outeAction.weightedBackendService.backendServiceWeightAction[].headerAction
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Option<Box<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderAction>>,
    /// The rules for determining a match.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matchRules")]
    pub r#match_rules: Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRule>>,
    /// For routeRules within a given pathMatcher, priority determines the order
    /// in which load balancer will interpret routeRules. RouteRules are evaluated
    /// in order of priority, from the lowest to highest number. The priority of
    /// a rule decreases as its number increases (1, 2, 3, N+1). The first rule
    /// that matches the request is applied.
    /// You cannot configure two or more routeRules with the same priority.
    /// Priority for each rule must be set to a number between 0 and
    /// 2147483647 inclusive.
    /// Priority numbers can have gaps, which enable you to add or remove rules
    /// in the future without affecting the rest of the rules. For example,
    /// 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which
    /// you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the
    /// future without any impact on existing rules.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// In response to a matching matchRule, the load balancer performs advanced routing
    /// actions like URL rewrites, header transformations, etc. prior to forwarding the
    /// request to the selected backend. If  routeAction specifies any
    /// weightedBackendServices, service must not be set. Conversely if service is set,
    /// routeAction cannot contain any  weightedBackendServices. Only one of routeAction
    /// or urlRedirect must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "routeAction")]
    pub r#route_action: Option<Box<super::super::types::compute::UrlMapPathMatcherRouteRuleRouteAction>>,
    /// The backend service resource to which traffic is
    /// directed if this rule is matched. If routeAction is additionally specified,
    /// advanced routing actions like URL Rewrites, etc. take effect prior to sending
    /// the request to the backend. However, if service is specified, routeAction cannot
    /// contain any weightedBackendService s. Conversely, if routeAction specifies any
    /// weightedBackendServices, service must not be specified. Only one of urlRedirect,
    /// service or routeAction.weightedBackendService must be set.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Option<String>,
    /// When this rule is matched, the request is redirected to a URL specified by
    /// urlRedirect. If urlRedirect is specified, service or routeAction must not be
    /// set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "urlRedirect")]
    pub r#url_redirect: Option<Box<super::super::types::compute::UrlMapPathMatcherRouteRuleUrlRedirect>>,
}
