#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MaintenanceWindowTaskTaskInvocationParametersLambdaParameters {
    /// Pass client-specific information to the Lambda function that you are invoking.
    #[builder(into)]
    #[serde(rename = "clientContext")]
    pub r#client_context: Option<String>,
    /// JSON to provide to your Lambda function as input.
    #[builder(into)]
    #[serde(rename = "payload")]
    pub r#payload: Option<String>,
    /// Specify a Lambda function version or alias name.
    #[builder(into)]
    #[serde(rename = "qualifier")]
    pub r#qualifier: Option<String>,
}
