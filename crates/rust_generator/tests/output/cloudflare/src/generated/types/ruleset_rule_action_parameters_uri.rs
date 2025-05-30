#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersUri {
    #[builder(into, default)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<bool>>,
    /// URI path configuration when performing a URL rewrite.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::types::RulesetRuleActionParametersUriPath>>,
    /// Query string configuration when performing a URL rewrite.
    #[builder(into, default)]
    #[serde(rename = "query")]
    pub r#query: Box<Option<super::types::RulesetRuleActionParametersUriQuery>>,
}
