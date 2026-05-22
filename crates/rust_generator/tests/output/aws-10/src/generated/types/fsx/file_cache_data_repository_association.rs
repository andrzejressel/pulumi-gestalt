#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FileCacheDataRepositoryAssociation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "associationId",
                    &self.r#association_id,
                ),
                to_pulumi_object_field(
                    "dataRepositoryPath",
                    &self.r#data_repository_path,
                ),
                to_pulumi_object_field(
                    "dataRepositorySubdirectories",
                    &self.r#data_repository_subdirectories,
                ),
                to_pulumi_object_field(
                    "fileCacheId",
                    &self.r#file_cache_id,
                ),
                to_pulumi_object_field(
                    "fileCachePath",
                    &self.r#file_cache_path,
                ),
                to_pulumi_object_field(
                    "fileSystemId",
                    &self.r#file_system_id,
                ),
                to_pulumi_object_field(
                    "fileSystemPath",
                    &self.r#file_system_path,
                ),
                to_pulumi_object_field(
                    "importedFileChunkSize",
                    &self.r#imported_file_chunk_size,
                ),
                to_pulumi_object_field(
                    "nfs",
                    &self.r#nfs,
                ),
                to_pulumi_object_field(
                    "resourceArn",
                    &self.r#resource_arn,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FileCacheDataRepositoryAssociation {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#association_id: {
                        let field_value = match fields_map.get("associationId") {
                            Some(value) => value,
                            None => bail!("Missing field 'associationId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_repository_path: {
                        let field_value = match fields_map.get("dataRepositoryPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataRepositoryPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_repository_subdirectories: {
                        let field_value = match fields_map.get("dataRepositorySubdirectories") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataRepositorySubdirectories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_cache_id: {
                        let field_value = match fields_map.get("fileCacheId") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileCacheId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_cache_path: {
                        let field_value = match fields_map.get("fileCachePath") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileCachePath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system_id: {
                        let field_value = match fields_map.get("fileSystemId") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileSystemId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_system_path: {
                        let field_value = match fields_map.get("fileSystemPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'fileSystemPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#imported_file_chunk_size: {
                        let field_value = match fields_map.get("importedFileChunkSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'importedFileChunkSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs: {
                        let field_value = match fields_map.get("nfs") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_arn: {
                        let field_value = match fields_map.get("resourceArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
