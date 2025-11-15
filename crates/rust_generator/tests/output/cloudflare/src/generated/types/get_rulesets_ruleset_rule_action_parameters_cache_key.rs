#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersCacheKey {
    /// Cache by device type. Conflicts with "custom_key.user.device_type".
    #[builder(into)]
    #[serde(rename = "cacheByDeviceType")]
    pub r#cache_by_device_type: Option<bool>,
    /// Cache deception armor.
    #[builder(into)]
    #[serde(rename = "cacheDeceptionArmor")]
    pub r#cache_deception_armor: Option<bool>,
    /// Custom key parameters for the request.
    #[builder(into)]
    #[serde(rename = "customKey")]
    pub r#custom_key: Option<Box<super::types::GetRulesetsRulesetRuleActionParametersCacheKeyCustomKey>>,
    /// Ignore query strings order.
    #[builder(into)]
    #[serde(rename = "ignoreQueryStringsOrder")]
    pub r#ignore_query_strings_order: Option<bool>,
}
