#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleHeaderMatch {
    /// The value of the header should exactly match contents of exactMatch.
    #[builder(into)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Option<String>,
    /// The header name to match on.
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// If set to false (default), the headerMatch is considered a match if the match criteria above are met.
    /// If set to true, the headerMatch is considered a match if the match criteria above are NOT met.
    #[builder(into)]
    #[serde(rename = "invertMatch")]
    pub r#invert_match: Option<bool>,
    /// The value of the header must start with the contents of prefixMatch.
    #[builder(into)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Option<String>,
    /// A header with the contents of headerName must exist. The match takes place whether or not the request's header has a value.
    #[builder(into)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Option<bool>,
    /// The value of the header must end with the contents of suffixMatch.
    #[builder(into)]
    #[serde(rename = "suffixMatch")]
    pub r#suffix_match: Option<String>,
}
