#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionEksPropertyPodPropertyContainerVolumeMount {
    /// The path on the container where the volume is mounted.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: String,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// If this value is true, the container has read-only access to the volume.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: bool,
}
