#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentLastUpdatedError {
    #[builder(into)]
    #[serde(rename = "errorCode")]
    pub r#error_code: Option<String>,
    #[builder(into)]
    #[serde(rename = "errorMessage")]
    pub r#error_message: Option<String>,
}
