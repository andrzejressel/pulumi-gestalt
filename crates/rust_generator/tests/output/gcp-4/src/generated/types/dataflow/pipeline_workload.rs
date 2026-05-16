#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipelineWorkload {
    /// Template information and additional parameters needed to launch a Dataflow job using the flex launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchflextemplaterequest
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataflowFlexTemplateRequest")]
    pub r#dataflow_flex_template_request: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowFlexTemplateRequest>>,
    /// Template information and additional parameters needed to launch a Dataflow job using the standard launch API.
    /// https://cloud.google.com/dataflow/docs/reference/data-pipelines/rest/v1/projects.locations.pipelines#launchtemplaterequest
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "dataflowLaunchTemplateRequest")]
    pub r#dataflow_launch_template_request: Option<Box<super::super::types::dataflow::PipelineWorkloadDataflowLaunchTemplateRequest>>,
}
