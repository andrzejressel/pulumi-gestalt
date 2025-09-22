#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationRuleCriteriaUserDefinedField {
    #[builder(into)]
    #[serde(rename = "comparison")]
    pub r#comparison: String,
    /// The key of the map filter.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
