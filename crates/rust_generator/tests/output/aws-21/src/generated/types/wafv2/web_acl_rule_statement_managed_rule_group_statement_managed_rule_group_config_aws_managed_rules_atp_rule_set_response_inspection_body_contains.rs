#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAtpRuleSetResponseInspectionBodyContains {
    /// Strings in the body of the response that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureStrings")]
    pub r#failure_strings: Vec<String>,
    /// Strings in the body of the response that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successStrings")]
    pub r#success_strings: Vec<String>,
}
