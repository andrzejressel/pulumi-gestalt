#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleAction {
    /// Describes the rule action that returns a custom HTTP response.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<super::super::types::vpclattice::ListenerRuleActionFixedResponse>>,
    /// The forward action. Traffic that matches the rule is forwarded to the specified target groups.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Box<Option<super::super::types::vpclattice::ListenerRuleActionForward>>,
}
