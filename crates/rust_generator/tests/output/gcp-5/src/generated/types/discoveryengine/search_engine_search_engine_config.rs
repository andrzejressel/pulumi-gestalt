#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SearchEngineSearchEngineConfig {
    /// The add-on that this search engine enables.
    /// Each value may be one of: `SEARCH_ADD_ON_LLM`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "searchAddOns")]
    pub r#search_add_ons: Option<Vec<String>>,
    /// The search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified.
    /// Default value is `SEARCH_TIER_STANDARD`.
    /// Possible values are: `SEARCH_TIER_STANDARD`, `SEARCH_TIER_ENTERPRISE`.
    #[builder(into)]
    #[serde(rename = "searchTier")]
    pub r#search_tier: Option<String>,
}
