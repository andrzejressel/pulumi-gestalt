#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetImagesImageOsDisk {
    /// the URI in Azure storage of the blob used to create the image.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: String,
    /// the caching mode for the Data Disk.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// the ID of the Disk Encryption Set used to encrypt this image.
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: String,
    /// the ID of the Managed Disk used as the Data Disk Image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: String,
    /// the State of the OS used in the Image.
    #[builder(into)]
    #[serde(rename = "osState")]
    pub r#os_state: String,
    /// the type of Operating System used on the OS Disk.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: String,
    /// the size of this Data Disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: i32,
}
