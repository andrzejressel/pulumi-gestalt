#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRule {
    /// The rule that specifies conditions when findings of infoTypes specified in InspectionRuleSet are removed from results.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "exclusionRule")]
    pub r#exclusion_rule: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRule>>,
    /// Hotword-based detection rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hotwordRule")]
    pub r#hotword_rule: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleHotwordRule>>,
}
