#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionCacheBehaviorSettingsForwardedQueryStrings {
    /// Indicates whether the distribution forwards and caches based on query strings.
    #[builder(into)]
    #[serde(rename = "option")]
    pub r#option: Option<bool>,
    /// The specific query strings that the distribution forwards to the origin.
    #[builder(into)]
    #[serde(rename = "queryStringsAllowedLists")]
    pub r#query_strings_allowed_lists: Option<Vec<String>>,
}
