#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocation {
    /// The ID of the file system, assigned by Amazon EFS.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Option<String>,
    /// The pathname for the folder being used by a workflow.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
}
