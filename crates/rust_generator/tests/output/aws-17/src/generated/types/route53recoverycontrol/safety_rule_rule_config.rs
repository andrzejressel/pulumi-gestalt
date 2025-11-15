#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SafetyRuleRuleConfig {
    /// Logical negation of the rule.
    #[builder(into)]
    #[serde(rename = "inverted")]
    pub r#inverted: bool,
    /// Number of controls that must be set when you specify an `ATLEAST` type rule.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: i32,
    /// Rule type. Valid values are `ATLEAST`, `AND`, and `OR`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
