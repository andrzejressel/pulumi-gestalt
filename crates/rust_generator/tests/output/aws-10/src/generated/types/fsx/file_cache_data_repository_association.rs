#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FileCacheDataRepositoryAssociation {
    #[builder(into)]
    #[serde(rename = "associationId")]
    pub r#association_id: Option<String>,
    /// The path to the S3 or NFS data repository that links to the cache.
    #[builder(into)]
    #[serde(rename = "dataRepositoryPath")]
    pub r#data_repository_path: String,
    /// A list of NFS Exports that will be linked with this data repository association. The Export paths are in the format /exportpath1. To use this parameter, you must configure DataRepositoryPath as the domain name of the NFS file system. The NFS file system domain name in effect is the root of the subdirectories. Note that DataRepositorySubdirectories is not supported for S3 data repositories. Max of 500.
    #[builder(into)]
    #[serde(rename = "dataRepositorySubdirectories")]
    pub r#data_repository_subdirectories: Option<Vec<String>>,
    /// The system-generated, unique ID of the cache.
    #[builder(into)]
    #[serde(rename = "fileCacheId")]
    pub r#file_cache_id: Option<String>,
    /// A path on the cache that points to a high-level directory (such as /ns1/) or subdirectory (such as /ns1/subdir/) that will be mapped 1-1 with DataRepositoryPath. The leading forward slash in the name is required. Two data repository associations cannot have overlapping cache paths. For example, if a data repository is associated with cache path /ns1/, then you cannot link another data repository with cache path /ns1/ns2. This path specifies where in your cache files will be exported from. This cache directory can be linked to only one data repository, and no data repository other can be linked to the directory. Note: The cache path can only be set to root (/) on an NFS DRA when DataRepositorySubdirectories is specified. If you specify root (/) as the cache path, you can create only one DRA on the cache. The cache path cannot be set to root (/) for an S3 DRA.
    #[builder(into)]
    #[serde(rename = "fileCachePath")]
    pub r#file_cache_path: String,
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "fileSystemPath")]
    pub r#file_system_path: Option<String>,
    #[builder(into)]
    #[serde(rename = "importedFileChunkSize")]
    pub r#imported_file_chunk_size: Option<i32>,
    /// (Optional) See the `nfs` configuration block.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Option<Vec<super::super::types::fsx::FileCacheDataRepositoryAssociationNf>>,
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Option<String>,
    /// A map of tags to assign to the file cache. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
}
