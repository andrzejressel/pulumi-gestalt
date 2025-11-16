#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkInsightsAnalysisAlternatePathHint {
    /// The Amazon Resource Name (ARN) of the component.
    #[builder(into)]
    #[serde(rename = "componentArn")]
    pub r#component_arn: Option<String>,
    /// The ID of the component.
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: Option<String>,
}
