#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationSource {
    /// Type of data that will trigger this automation. Must be one of `Alerts`, `Assessments`, `AssessmentsSnapshot`, `RegulatoryComplianceAssessment`, `RegulatoryComplianceAssessmentSnapshot`, `SecureScoreControls`, `SecureScoreControlsSnapshot`, `SecureScores`, `SecureScoresSnapshot`, `SubAssessments` or `SubAssessmentsSnapshot`. Note. assessments are also referred to as recommendations
    #[builder(into)]
    #[serde(rename = "eventSource")]
    pub r#event_source: String,
    /// A set of rules which evaluate upon event and data interception. This is defined in one or more `rule_set` blocks as defined below.
    /// 
    /// > **NOTE:** When multiple `rule_set` block are provided, a logical 'OR' is applied to the evaluation of them.
    #[builder(into)]
    #[serde(rename = "ruleSets")]
    pub r#rule_sets: Option<Vec<super::super::types::securitycenter::AutomationSourceRuleSet>>,
}
