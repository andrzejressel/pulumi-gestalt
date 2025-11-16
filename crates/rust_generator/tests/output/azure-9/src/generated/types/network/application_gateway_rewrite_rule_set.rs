#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayRewriteRuleSet {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Unique name of the rewrite rule set block
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `rewrite_rule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "rewriteRules")]
    pub r#rewrite_rules: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRule>>,
}
