#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLinuxParameterTmpf {
    /// The absolute file path in the container where the tmpfs volume is mounted.
    #[builder(into)]
    #[serde(rename = "containerPath")]
    pub r#container_path: String,
    /// The list of tmpfs volume mount options.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Vec<String>,
    /// The size (in MiB) of the tmpfs volume.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
}
