#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceActionDefinition {
    /// ARN of the role that performs the self-service actions on your behalf. For example, `arn:aws:iam::12345678910:role/ActionRole`. To reuse the provisioned product launch role, set to `LAUNCH_ROLE`.
    #[builder(into)]
    #[serde(rename = "assumeRole")]
    pub r#assume_role: Option<String>,
    /// Name of the SSM document. For example, `AWS-RestartEC2Instance`. If you are using a shared SSM document, you must provide the ARN instead of the name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// List of parameters in JSON format. For example: `[{\"Name\":\"InstanceId\",\"Type\":\"TARGET\"}]` or `[{\"Name\":\"InstanceId\",\"Type\":\"TEXT_VALUE\"}]`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
    /// Service action definition type. Valid value is `SSM_AUTOMATION`. Default is `SSM_AUTOMATION`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// SSM document version. For example, `1`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
