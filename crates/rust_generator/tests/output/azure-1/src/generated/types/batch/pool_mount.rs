#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolMount {
    /// A `azure_blob_file_system` block defined as below.
    #[builder(into)]
    #[serde(rename = "azureBlobFileSystem")]
    pub r#azure_blob_file_system: Option<Box<super::super::types::batch::PoolMountAzureBlobFileSystem>>,
    /// A `azure_file_share` block defined as below.
    #[builder(into)]
    #[serde(rename = "azureFileShares")]
    pub r#azure_file_shares: Option<Vec<super::super::types::batch::PoolMountAzureFileShare>>,
    /// A `cifs_mount` block defined as below.
    #[builder(into)]
    #[serde(rename = "cifsMounts")]
    pub r#cifs_mounts: Option<Vec<super::super::types::batch::PoolMountCifsMount>>,
    /// A `nfs_mount` block defined as below.
    #[builder(into)]
    #[serde(rename = "nfsMounts")]
    pub r#nfs_mounts: Option<Vec<super::super::types::batch::PoolMountNfsMount>>,
}
