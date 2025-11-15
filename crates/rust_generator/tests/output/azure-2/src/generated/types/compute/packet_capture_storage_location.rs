#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PacketCaptureStorageLocation {
    /// A valid local path on the target Virtual Machine. Must include the name of the capture file (*.cap). For Linux Virtual Machines it must start with `/var/captures`.
    #[builder(into)]
    #[serde(rename = "filePath")]
    pub r#file_path: Option<String>,
    /// The ID of the storage account where the packet capture sessions should be saved to.
    /// 
    /// > **NOTE:** At least one of `file_path` or `storage_account_id` must be specified.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Option<String>,
    /// The URI of the storage path where the packet capture sessions are saved to.
    #[builder(into)]
    #[serde(rename = "storagePath")]
    pub r#storage_path: Option<String>,
}
