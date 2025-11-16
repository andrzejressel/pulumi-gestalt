#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
    /// For satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL.
    #[builder(into)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Option<String>,
    /// Specifies a list of header match criteria, all of which must match corresponding headers in the request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleHeaderMatch>>,
    /// Specifies that prefixMatch and fullPathMatch matches are case sensitive.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// For satisfying the matchRule condition, the path of the request
    /// must match the wildcard pattern specified in pathTemplateMatch
    /// after removing any query parameters and anchor that may be part
    /// of the original URL.
    /// pathTemplateMatch must be between 1 and 255 characters
    /// (inclusive).  The pattern specified by pathTemplateMatch may
    /// have at most 5 wildcard operators and at most 5 variable
    /// captures in total.
    #[builder(into)]
    #[serde(rename = "pathTemplateMatch")]
    pub r#path_template_match: Option<String>,
    /// For satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// Specifies a list of query parameter match criteria, all of which must match corresponding query parameters in the request.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "queryParameterMatches")]
    pub r#query_parameter_matches: Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleQueryParameterMatch>>,
}
