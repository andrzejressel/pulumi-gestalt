#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KxClusterSavedownStorageConfiguration {
    /// Size of temporary storage in gigabytes. Must be between 10 and 16000.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<i32>,
    /// Type of writeable storage space for temporarily storing your savedown data. The valid values are:
    /// * SDS01 - This type represents 3000 IOPS and io2 ebs volume type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// The name of the kdb volume that you want to use as writeable save-down storage for clusters.
    #[builder(into)]
    #[serde(rename = "volumeName")]
    pub r#volume_name: Option<String>,
}
