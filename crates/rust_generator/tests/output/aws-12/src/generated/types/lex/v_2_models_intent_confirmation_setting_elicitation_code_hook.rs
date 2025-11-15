#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsIntentConfirmationSettingElicitationCodeHook {
    /// Whether a Lambda function should be invoked for the dialog.
    #[builder(into)]
    #[serde(rename = "enableCodeHookInvocation")]
    pub r#enable_code_hook_invocation: Option<bool>,
    /// Label that indicates the dialog step from which the dialog code hook is happening.
    #[builder(into)]
    #[serde(rename = "invocationLabel")]
    pub r#invocation_label: Option<String>,
}
