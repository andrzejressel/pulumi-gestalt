#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSpringCloudAppPersistentDisk {
    /// The mount path of the persistent disk.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: String,
    /// The size of the persistent disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: i32,
}
