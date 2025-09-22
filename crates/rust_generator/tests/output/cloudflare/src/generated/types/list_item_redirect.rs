#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListItemRedirect {
    /// Whether the redirect also matches subdomains of the source url.
    #[builder(into)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Option<bool>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[builder(into)]
    #[serde(rename = "preservePathSuffix")]
    pub r#preserve_path_suffix: Option<bool>,
    /// Whether the redirect target url should keep the query string of the request's url.
    #[builder(into)]
    #[serde(rename = "preserveQueryString")]
    pub r#preserve_query_string: Option<bool>,
    /// The source url of the redirect.
    #[builder(into)]
    #[serde(rename = "sourceUrl")]
    pub r#source_url: String,
    /// The status code to be used when redirecting a request.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
    /// Whether the redirect also matches subpaths of the source url.
    #[builder(into)]
    #[serde(rename = "subpathMatching")]
    pub r#subpath_matching: Option<bool>,
    /// The target url of the redirect.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}
