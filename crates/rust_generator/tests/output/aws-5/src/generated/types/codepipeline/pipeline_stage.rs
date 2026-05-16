#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineStage {
    /// The action(s) to include in the stage. Defined as an `action` block below
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Vec<super::super::types::codepipeline::PipelineStageAction>,
    /// The name of the stage.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
