#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RulesEngineRuleAction {
    /// A `request_header` block as defined below.
    #[builder(into)]
    #[serde(rename = "requestHeaders")]
    pub r#request_headers: Option<Vec<super::super::types::frontdoor::RulesEngineRuleActionRequestHeader>>,
    /// A `response_header` block as defined below.
    #[builder(into)]
    #[serde(rename = "responseHeaders")]
    pub r#response_headers: Option<Vec<super::super::types::frontdoor::RulesEngineRuleActionResponseHeader>>,
}
