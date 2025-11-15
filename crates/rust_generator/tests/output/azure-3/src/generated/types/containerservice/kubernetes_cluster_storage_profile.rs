#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterStorageProfile {
    /// Is the Blob CSI driver enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "blobDriverEnabled")]
    pub r#blob_driver_enabled: Option<bool>,
    /// Is the Disk CSI driver enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "diskDriverEnabled")]
    pub r#disk_driver_enabled: Option<bool>,
    /// Is the File CSI driver enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "fileDriverEnabled")]
    pub r#file_driver_enabled: Option<bool>,
    /// Is the Snapshot Controller enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "snapshotControllerEnabled")]
    pub r#snapshot_controller_enabled: Option<bool>,
}
