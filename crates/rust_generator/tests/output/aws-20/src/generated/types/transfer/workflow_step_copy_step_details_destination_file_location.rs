#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowStepCopyStepDetailsDestinationFileLocation {
    /// Specifies the details for the EFS file being copied.
    #[builder(into)]
    #[serde(rename = "efsFileLocation")]
    pub r#efs_file_location: Option<Box<super::super::types::transfer::WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocation>>,
    /// Specifies the details for the S3 file being copied.
    #[builder(into)]
    #[serde(rename = "s3FileLocation")]
    pub r#s_3_file_location: Option<Box<super::super::types::transfer::WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocation>>,
}
