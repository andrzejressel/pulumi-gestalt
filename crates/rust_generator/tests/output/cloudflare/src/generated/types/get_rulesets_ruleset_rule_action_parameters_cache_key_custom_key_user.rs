#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKeyCustomKeyUser {
    /// Add device type to the custom key. Conflicts with "cache_key.cache_by_device_type".
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<bool>,
    /// Add geo data to the custom key.
    #[builder(into)]
    #[serde(rename = "geo")]
    pub r#geo: Option<bool>,
    /// Add language data to the custom key.
    #[builder(into)]
    #[serde(rename = "lang")]
    pub r#lang: Option<bool>,
}
