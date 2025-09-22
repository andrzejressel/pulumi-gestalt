#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleUrlRedirect {
    /// The host that will be used in the redirect response instead of the one that was supplied in the request.
    #[builder(into)]
    #[serde(rename = "hostRedirect")]
    pub r#host_redirect: Option<String>,
    /// If set to true, the URL scheme in the redirected request is set to https. If set to false, the URL scheme of the redirected request will remain the same as that of the request.
    /// This can only be set if there is at least one (1) edgeSslCertificate set on the service.
    #[builder(into)]
    #[serde(rename = "httpsRedirect")]
    pub r#https_redirect: Option<bool>,
    /// The path that will be used in the redirect response instead of the one that was supplied in the request.
    /// pathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.
    /// The path value must be between 1 and 1024 characters.
    #[builder(into)]
    #[serde(rename = "pathRedirect")]
    pub r#path_redirect: Option<String>,
    /// The prefix that replaces the prefixMatch specified in the routeRule, retaining the remaining portion of the URL before redirecting the request.
    /// prefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect.
    #[builder(into)]
    #[serde(rename = "prefixRedirect")]
    pub r#prefix_redirect: Option<String>,
    /// The HTTP Status code to use for this RedirectAction.
    /// The supported values are:
    /// - `MOVED_PERMANENTLY_DEFAULT`, which is the default value and corresponds to 301.
    /// - `FOUND`, which corresponds to 302.
    /// - `SEE_OTHER` which corresponds to 303.
    /// - `TEMPORARY_REDIRECT`, which corresponds to 307. in this case, the request method will be retained.
    /// - `PERMANENT_REDIRECT`, which corresponds to 308. in this case, the request method will be retained.
    /// Possible values are: `MOVED_PERMANENTLY_DEFAULT`, `FOUND`, `SEE_OTHER`, `TEMPORARY_REDIRECT`, `PERMANENT_REDIRECT`.
    #[builder(into)]
    #[serde(rename = "redirectResponseCode")]
    pub r#redirect_response_code: Option<String>,
    /// If set to true, any accompanying query portion of the original URL is removed prior to redirecting the request. If set to false, the query portion of the original URL is retained.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "stripQuery")]
    pub r#strip_query: Option<bool>,
}
