#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNodeConfigSecondaryBootDisk {
    /// Path to disk image to create the secondary boot disk from. After using the [gke-disk-image-builder](https://github.com/GoogleCloudPlatform/ai-on-gke/tree/main/tools/gke-disk-image-builder), this argument should be `global/images/DISK_IMAGE_NAME`.
    #[builder(into)]
    #[serde(rename = "diskImage")]
    pub r#disk_image: String,
    /// Mode for how the secondary boot disk is used. An example mode is `CONTAINER_IMAGE_CACHE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}
