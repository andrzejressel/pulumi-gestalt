#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRule {
    /// Dictionary which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dictionary")]
    pub r#dictionary: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleDictionary>>,
    /// Drop if the hotword rule is contained in the proximate context.
    /// For tabular data, the context includes the column name.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeByHotword")]
    pub r#exclude_by_hotword: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeByHotword>>,
    /// Set of infoTypes for which findings would affect this rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludeInfoTypes")]
    pub r#exclude_info_types: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleExcludeInfoTypes>>,
    /// How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType
    /// Possible values are: `MATCHING_TYPE_FULL_MATCH`, `MATCHING_TYPE_PARTIAL_MATCH`, `MATCHING_TYPE_INVERSE_MATCH`.
    #[builder(into)]
    #[serde(rename = "matchingType")]
    pub r#matching_type: String,
    /// Regular expression which defines the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Option<Box<super::super::types::dataloss::PreventionInspectTemplateInspectConfigRuleSetRuleExclusionRuleRegex>>,
}
