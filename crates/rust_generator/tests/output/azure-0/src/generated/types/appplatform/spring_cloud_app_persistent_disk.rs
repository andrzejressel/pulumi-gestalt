#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudAppPersistentDisk {
    /// Specifies the mount path of the persistent disk. Defaults to `/persistent`.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Option<String>,
    /// Specifies the size of the persistent disk in GB. Possible values are between `0` and `50`.
    #[builder(into)]
    #[serde(rename = "sizeInGb")]
    pub r#size_in_gb: i32,
}
