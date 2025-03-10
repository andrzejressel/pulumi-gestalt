#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointClientConnectOption {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[builder(into)]
    #[serde(rename = "lambdaFunctionArn")]
    pub r#lambda_function_arn: Box<String>,
}
