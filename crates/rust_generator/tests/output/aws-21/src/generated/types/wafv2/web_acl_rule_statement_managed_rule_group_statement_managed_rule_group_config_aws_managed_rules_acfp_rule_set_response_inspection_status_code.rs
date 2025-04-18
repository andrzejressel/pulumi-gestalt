#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetResponseInspectionStatusCode {
    /// Status codes in the response that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureCodes")]
    pub r#failure_codes: Box<Vec<i32>>,
    /// Status codes in the response that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successCodes")]
    pub r#success_codes: Box<Vec<i32>>,
}
