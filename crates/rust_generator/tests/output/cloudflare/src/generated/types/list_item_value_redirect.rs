#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListItemValueRedirect {
    /// Whether the redirect also matches subdomains of the source url. Available values: `disabled`, `enabled`.
    #[builder(into, default)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<String>>,
    /// Whether to preserve the path suffix when doing subpath matching. Available values: `disabled`, `enabled`.
    #[builder(into, default)]
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Box<Option<String>>,
    /// Whether the redirect target url should keep the query string of the request's url. Available values: `disabled`, `enabled`.
    #[builder(into, default)]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Box<Option<String>>,
    /// The source url of the redirect.
    #[builder(into)]
    #[serde(rename = "sourceUrl")]
    pub r#source_url: Box<String>,
    /// The status code to be used when redirecting a request.
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<i32>>,
    /// Whether the redirect also matches subpaths of the source url. Available values: `disabled`, `enabled`.
    #[builder(into, default)]
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Box<Option<String>>,
    /// The target url of the redirect.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Box<String>,
}
