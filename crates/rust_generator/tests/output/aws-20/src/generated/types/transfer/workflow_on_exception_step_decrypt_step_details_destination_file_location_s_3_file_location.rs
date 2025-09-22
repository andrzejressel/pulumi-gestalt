#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation {
    /// Specifies the S3 bucket for the customer input file.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// The name assigned to the file when it was created in S3. You use the object key to retrieve the object.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
}
