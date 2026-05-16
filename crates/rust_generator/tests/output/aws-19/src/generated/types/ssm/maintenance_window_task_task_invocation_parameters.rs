#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MaintenanceWindowTaskTaskInvocationParameters {
    /// The parameters for an AUTOMATION task type. Documented below.
    #[builder(into)]
    #[serde(rename = "automationParameters")]
    pub r#automation_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersAutomationParameters>>,
    /// The parameters for a LAMBDA task type. Documented below.
    #[builder(into)]
    #[serde(rename = "lambdaParameters")]
    pub r#lambda_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersLambdaParameters>>,
    /// The parameters for a RUN_COMMAND task type. Documented below.
    #[builder(into)]
    #[serde(rename = "runCommandParameters")]
    pub r#run_command_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters>>,
    /// The parameters for a STEP_FUNCTIONS task type. Documented below.
    #[builder(into)]
    #[serde(rename = "stepFunctionsParameters")]
    pub r#step_functions_parameters: Option<Box<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersStepFunctionsParameters>>,
}
