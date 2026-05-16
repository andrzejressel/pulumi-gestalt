#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolNodeConfigSecondaryBootDisk {
    /// Disk image to create the secondary boot disk from
    #[builder(into)]
    #[serde(rename = "diskImage")]
    pub r#disk_image: String,
    /// Mode for how the secondary boot disk is used.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}
