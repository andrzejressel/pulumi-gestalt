#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRuleProximity {
    /// Number of characters after the finding to consider.
    #[builder(into)]
    #[serde(rename = "windowAfter")]
    pub r#window_after: Option<i32>,
    /// Number of characters before the finding to consider.
    #[builder(into)]
    #[serde(rename = "windowBefore")]
    pub r#window_before: Option<i32>,
}
