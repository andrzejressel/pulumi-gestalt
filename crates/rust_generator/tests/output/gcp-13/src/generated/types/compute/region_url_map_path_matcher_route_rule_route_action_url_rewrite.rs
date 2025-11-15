#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionUrlMapPathMatcherRouteRuleRouteActionUrlRewrite {
    /// Before forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.
    /// The value must be from 1 to 255 characters.
    #[builder(into)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Option<String>,
    /// Before forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.
    /// The value must be from 1 to 1024 characters.
    #[builder(into)]
    #[serde(rename = "pathPrefixRewrite")]
    pub r#path_prefix_rewrite: Option<String>,
    /// Prior to forwarding the request to the selected origin, if the
    /// request matched a pathTemplateMatch, the matching portion of the
    /// request's path is replaced re-written using the pattern specified
    /// by pathTemplateRewrite.
    /// pathTemplateRewrite must be between 1 and 255 characters
    /// (inclusive), must start with a '/', and must only use variables
    /// captured by the route's pathTemplate matchers.
    /// pathTemplateRewrite may only be used when all of a route's
    /// MatchRules specify pathTemplate.
    /// Only one of pathPrefixRewrite and pathTemplateRewrite may be
    /// specified.
    #[builder(into)]
    #[serde(rename = "pathTemplateRewrite")]
    pub r#path_template_rewrite: Option<String>,
}
