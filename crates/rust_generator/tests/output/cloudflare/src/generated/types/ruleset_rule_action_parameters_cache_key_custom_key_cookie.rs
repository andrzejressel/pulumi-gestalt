#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleActionParametersCacheKeyCustomKeyCookie {
    /// List of cookies to check for presence in the custom key.
    #[builder(into)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    /// List of cookies to include in the custom key.
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
