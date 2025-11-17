#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EmailRoutingRuleMatcher {
    /// Field to match on. Required for `type` of `literal`.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    /// Type of matcher. Available values: `literal`, `all`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Value to match on. Required for `type` of `literal`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
