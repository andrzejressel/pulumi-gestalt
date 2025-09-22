#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyPolicySettingsLogScrubbingRule {
    /// Describes if the managed rule is in enabled state or disabled state. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "matchVariable")]
    pub r#match_variable: String,
    /// When matchVariable is a collection, operator used to specify which elements in the collection this rule applies to.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Option<String>,
    #[builder(into)]
    #[serde(rename = "selectorMatchOperator")]
    pub r#selector_match_operator: Option<String>,
}
