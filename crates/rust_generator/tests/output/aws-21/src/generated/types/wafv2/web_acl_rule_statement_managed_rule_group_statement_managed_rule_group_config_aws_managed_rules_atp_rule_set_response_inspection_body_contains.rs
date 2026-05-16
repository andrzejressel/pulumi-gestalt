#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
