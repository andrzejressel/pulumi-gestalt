#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerMount {
    /// Optional configuration for the bind type.
    #[builder(into)]
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<super::types::ContainerMountBindOptions>>,
    /// Whether the mount should be read-only.
    #[builder(into)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Option<bool>,
    /// Mount source (e.g. a volume name, a host path).
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// Container path
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: String,
    /// Optional configuration for the tmpfs type.
    #[builder(into)]
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<super::types::ContainerMountTmpfsOptions>>,
    /// The mount type
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// Optional configuration for the volume type.
    #[builder(into)]
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<super::types::ContainerMountVolumeOptions>>,
}
