#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesetRuleActionParametersEdgeTtl {
    /// Default edge TTL.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: Option<i32>,
    /// Mode of the edge TTL. Available values: `override_origin`, `respect_origin`, `bypass_by_default`
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// Edge TTL for the status codes.
    #[builder(into)]
    #[serde(rename = "statusCodeTtls")]
    pub r#status_code_ttls: Option<Vec<super::types::RulesetRuleActionParametersEdgeTtlStatusCodeTtl>>,
}
