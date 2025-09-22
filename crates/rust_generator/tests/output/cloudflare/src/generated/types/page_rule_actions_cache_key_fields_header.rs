#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PageRuleActionsCacheKeyFieldsHeader {
    /// Check for presence of specified HTTP headers, without including their actual values.
    #[builder(into)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    /// Exclude these HTTP headers from Cache Key. Currently, only the `Origin` header can be excluded.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// Use values of specified HTTP headers in Cache Key. Please refer to [Support article](https://support.cloudflare.com/hc/en-us/articles/115004290387-Creating-Cache-Keys) for the list of HTTP headers that cannot be included. The `Origin` header is always included unless explicitly excluded.
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
