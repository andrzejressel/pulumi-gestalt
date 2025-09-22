#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RemediationConfigurationExecutionControlsSsmControls {
    /// Maximum percentage of remediation actions allowed to run in parallel on the non-compliant resources for that specific rule. The default value is 10%.
    #[builder(into)]
    #[serde(rename = "concurrentExecutionRatePercentage")]
    pub r#concurrent_execution_rate_percentage: Option<i32>,
    /// Percentage of errors that are allowed before SSM stops running automations on non-compliant resources for that specific rule. The default is 50%.
    #[builder(into)]
    #[serde(rename = "errorPercentage")]
    pub r#error_percentage: Option<i32>,
}
