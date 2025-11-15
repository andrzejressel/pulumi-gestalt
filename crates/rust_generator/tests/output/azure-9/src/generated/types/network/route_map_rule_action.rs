#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteMapRuleAction {
    /// A `parameter` block as defined below. Required if `type` is anything other than `Drop`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::network::RouteMapRuleActionParameter>>,
    /// The type of the action to be taken. Possible values are `Add`, `Drop`, `Remove`, `Replace` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
