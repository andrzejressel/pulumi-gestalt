#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PageRuleActionsCacheKeyFieldsQueryString {
    /// Exclude these query string parameters from Cache Key.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// `false` (default) - all query string parameters are used for Cache Key, unless explicitly excluded; `true` - all query string parameters are ignored; value should be `false` if any of `exclude` or `include` is non-empty.
    #[builder(into)]
    #[serde(rename = "ignore")]
    pub r#ignore: Option<bool>,
    /// Only use values of specified query string parameters in Cache Key.
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
