#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssessmentScope {
    /// Amazon Web Services accounts that are in scope for the assessment. See `aws_accounts` below.
    #[builder(into)]
    #[serde(rename = "awsAccounts")]
    pub r#aws_accounts: Option<Vec<super::super::types::auditmanager::AssessmentScopeAwsAccount>>,
    /// Amazon Web Services services that are included in the scope of the assessment. See `aws_services` below.
    #[builder(into)]
    #[serde(rename = "awsServices")]
    pub r#aws_services: Option<Vec<super::super::types::auditmanager::AssessmentScopeAwsService>>,
}
