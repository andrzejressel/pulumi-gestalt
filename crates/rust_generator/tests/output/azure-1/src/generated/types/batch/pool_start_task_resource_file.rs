#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolStartTaskResourceFile {
    /// The storage container name in the auto storage account.
    #[builder(into, default)]
    #[serde(rename = "autoStorageContainerName")]
    pub r#auto_storage_container_name: Box<Option<String>>,
    /// The blob prefix to use when downloading blobs from an Azure Storage container. Only the blobs whose names begin with the specified prefix will be downloaded. The property is valid only when `auto_storage_container_name` or `storage_container_url` is used. This prefix can be a partial filename or a subdirectory. If a prefix is not specified, all the files in the container will be downloaded.
    #[builder(into, default)]
    #[serde(rename = "blobPrefix")]
    pub r#blob_prefix: Box<Option<String>>,
    /// The file permission mode represented as a string in octal format (e.g. `"0644"`). This property applies only to files being downloaded to Linux compute nodes. It will be ignored if it is specified for a `resource_file` which will be downloaded to a Windows node. If this property is not specified for a Linux node, then a default value of 0770 is applied to the file.
    #[builder(into, default)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Box<Option<String>>,
    /// The location on the compute node to which to download the file, relative to the task's working directory. If the `http_url` property is specified, the `file_path` is required and describes the path which the file will be downloaded to, including the filename. Otherwise, if the `auto_storage_container_name` or `storage_container_url` property is specified, `file_path` is optional and is the directory to download the files to. In the case where `file_path` is used as a directory, any directory structure already associated with the input data will be retained in full and appended to the specified filePath directory. The specified relative path cannot break out of the task's working directory (for example by using '..').
    #[builder(into, default)]
    #[serde(rename = "filePath")]
    pub r#file_path: Box<Option<String>>,
    /// The URL of the file to download. If the URL is Azure Blob Storage, it must be readable using anonymous access; that is, the Batch service does not present any credentials when downloading the blob. There are two ways to get such a URL for a blob in Azure storage: include a Shared Access Signature (SAS) granting read permissions on the blob, or set the ACL for the blob or its container to allow public access.
    #[builder(into, default)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: Box<Option<String>>,
    /// The URL of the blob container within Azure Blob Storage. This URL must be readable and listable using anonymous access; that is, the Batch service does not present any credentials when downloading the blob. There are two ways to get such a URL for a blob in Azure storage: include a Shared Access Signature (SAS) granting read and list permissions on the blob, or set the ACL for the blob or its container to allow public access.
    #[builder(into, default)]
    #[serde(rename = "storageContainerUrl")]
    pub r#storage_container_url: Box<Option<String>>,
    /// An identity reference from pool's user assigned managed identity list.
    /// 
    /// > **Please Note:** Exactly one of `auto_storage_container_name`, `storage_container_url` and `auto_user` must be specified.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
