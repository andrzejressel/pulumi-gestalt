#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationGatewayRewriteRuleSetRewriteRuleCondition {
    /// Perform a case in-sensitive comparison. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// Negate the result of the condition evaluation. Defaults to `false`
    #[builder(into)]
    #[serde(rename = "negate")]
    pub r#negate: Option<bool>,
    /// The pattern, either fixed string or regular expression, that evaluates the truthfulness of the condition.
    #[builder(into)]
    #[serde(rename = "pattern")]
    pub r#pattern: String,
    /// The [variable](https://docs.microsoft.com/azure/application-gateway/rewrite-http-headers#server-variables) of the condition.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: String,
}
