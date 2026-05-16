#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentInitialResponseSettingCodeHook {
    /// Whether a dialog code hook is used when the intent is activated.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// Whether a Lambda function should be invoked for the dialog.
    #[builder(into)]
    #[serde(rename = "enableCodeHookInvocation")]
    pub r#enable_code_hook_invocation: bool,
    /// Label that indicates the dialog step from which the dialog code hook is happening.
    #[builder(into)]
    #[serde(rename = "invocationLabel")]
    pub r#invocation_label: Option<String>,
    /// Configuration block that contains the responses and actions that Amazon Lex takes after the Lambda function is complete. See `post_code_hook_specification`.
    #[builder(into)]
    #[serde(rename = "postCodeHookSpecification")]
    pub r#post_code_hook_specification: Box<super::super::types::lex::V2ModelsIntentInitialResponseSettingCodeHookPostCodeHookSpecification>,
}
