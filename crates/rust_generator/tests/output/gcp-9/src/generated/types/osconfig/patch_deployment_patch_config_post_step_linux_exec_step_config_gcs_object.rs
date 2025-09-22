#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentPatchConfigPostStepLinuxExecStepConfigGcsObject {
    /// Bucket of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Generation number of the Cloud Storage object. This is used to ensure that the ExecStep specified by this PatchJob does not change.
    #[builder(into)]
    #[serde(rename = "generationNumber")]
    pub r#generation_number: String,
    /// Name of the Cloud Storage object.
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: String,
}
