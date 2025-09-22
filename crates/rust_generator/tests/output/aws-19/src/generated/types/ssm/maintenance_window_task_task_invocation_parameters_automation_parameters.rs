#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MaintenanceWindowTaskTaskInvocationParametersAutomationParameters {
    /// The version of an Automation document to use during task execution.
    #[builder(into)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Option<String>,
    /// The parameters for the RUN_COMMAND task execution. Documented below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParametersParameter>>,
}
