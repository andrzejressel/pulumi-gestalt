#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementManagedRuleGroupStatementManagedRuleGroupConfigAwsManagedRulesAcfpRuleSetResponseInspectionJson {
    /// Values in the response header with the specified name that indicate a failed login attempt.
    #[builder(into)]
    #[serde(rename = "failureValues")]
    pub r#failure_values: Vec<String>,
    /// The identifier for the value to match against in the JSON.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: String,
    /// Values in the response header with the specified name that indicate a successful login attempt.
    #[builder(into)]
    #[serde(rename = "successValues")]
    pub r#success_values: Vec<String>,
}
