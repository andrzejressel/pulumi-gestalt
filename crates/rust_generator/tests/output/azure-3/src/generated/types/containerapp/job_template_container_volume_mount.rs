#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateContainerVolumeMount {
    /// The name of the volume to mount. This must match the name of a volume defined in the `volume` block.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The path within the container at which the volume should be mounted. Must not contain `:`.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
}
