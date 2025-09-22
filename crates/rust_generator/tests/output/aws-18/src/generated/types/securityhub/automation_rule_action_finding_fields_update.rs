#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationRuleActionFindingFieldsUpdate {
    /// The rule action updates the `Confidence` field of a finding.
    #[builder(into)]
    #[serde(rename = "confidence")]
    pub r#confidence: Option<i32>,
    /// The rule action updates the `Criticality` field of a finding.
    #[builder(into)]
    #[serde(rename = "criticality")]
    pub r#criticality: Option<i32>,
    /// A resource block that updates the note. Documented below.
    #[builder(into)]
    #[serde(rename = "note")]
    pub r#note: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateNote>>,
    /// A resource block that the rule action updates the `RelatedFindings` field of a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "relatedFindings")]
    pub r#related_findings: Option<Vec<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateRelatedFinding>>,
    /// A resource block that updates to the severity information for a finding. Documented below.
    #[builder(into)]
    #[serde(rename = "severity")]
    pub r#severity: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateSeverity>>,
    /// The rule action updates the `Types` field of a finding.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<String>>,
    /// The rule action updates the `UserDefinedFields` field of a finding.
    #[builder(into)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Option<std::collections::HashMap<String, String>>,
    /// The rule action updates the `VerificationState` field of a finding. The allowed values are the following `UNKNOWN`, `TRUE_POSITIVE`, `FALSE_POSITIVE` and `BENIGN_POSITIVE`.
    #[builder(into)]
    #[serde(rename = "verificationState")]
    pub r#verification_state: Option<String>,
    /// A resource block that is used to update information about the investigation into the finding. Documented below.
    #[builder(into)]
    #[serde(rename = "workflow")]
    pub r#workflow: Option<Box<super::super::types::securityhub::AutomationRuleActionFindingFieldsUpdateWorkflow>>,
}
