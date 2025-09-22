#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayRewriteRuleSetRewriteRule {
    /// One or more `condition` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleCondition>>,
    /// Unique name of the rewrite rule block
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `request_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "requestHeaderConfigurations")]
    pub r#request_header_configurations: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleRequestHeaderConfiguration>>,
    /// One or more `response_header_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "responseHeaderConfigurations")]
    pub r#response_header_configurations: Option<Vec<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleResponseHeaderConfiguration>>,
    /// Rule sequence of the rewrite rule that determines the order of execution in a set.
    #[builder(into)]
    #[serde(rename = "ruleSequence")]
    pub r#rule_sequence: i32,
    /// One `url` block as defined below
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<super::super::types::network::ApplicationGatewayRewriteRuleSetRewriteRuleUrl>>,
}
