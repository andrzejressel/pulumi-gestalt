#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPoolMount {
    /// A `azure_blob_file_system` block defined as below.
    #[builder(into)]
    #[serde(rename = "azureBlobFileSystems")]
    pub r#azure_blob_file_systems: Option<Vec<super::super::types::batch::GetPoolMountAzureBlobFileSystem>>,
    /// A `azure_file_share` block defined as below.
    #[builder(into)]
    #[serde(rename = "azureFileShares")]
    pub r#azure_file_shares: Option<Vec<super::super::types::batch::GetPoolMountAzureFileShare>>,
    /// A `cifs_mount` block defined as below.
    #[builder(into)]
    #[serde(rename = "cifsMounts")]
    pub r#cifs_mounts: Vec<super::super::types::batch::GetPoolMountCifsMount>,
    /// A `nfs_mount` block defined as below.
    #[builder(into)]
    #[serde(rename = "nfsMounts")]
    pub r#nfs_mounts: Vec<super::super::types::batch::GetPoolMountNfsMount>,
}
