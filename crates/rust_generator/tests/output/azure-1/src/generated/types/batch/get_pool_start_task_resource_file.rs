#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolStartTaskResourceFile {
    /// The storage container name in the auto storage account.
    #[builder(into)]
    #[serde(rename = "autoStorageContainerName")]
    pub r#auto_storage_container_name: String,
    /// The blob prefix used when downloading blobs from an Azure Storage container.
    #[builder(into)]
    #[serde(rename = "blobPrefix")]
    pub r#blob_prefix: String,
    /// The file permission mode attribute represented as a string in octal format (e.g. `"0644"`).
    #[builder(into)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: String,
    /// The location on the compute node to which to download the file, relative to the task's working directory. If the `http_url` property is specified, the `file_path` is required and describes the path which the file will be downloaded to, including the filename. Otherwise, if the `auto_storage_container_name` or `storage_container_url` property is specified.
    #[builder(into)]
    #[serde(rename = "filePath")]
    pub r#file_path: String,
    /// The URL of the file to download. If the URL is Azure Blob Storage, it must be readable using anonymous access.
    #[builder(into)]
    #[serde(rename = "httpUrl")]
    pub r#http_url: String,
    /// The URL of the blob container within Azure Blob Storage.
    #[builder(into)]
    #[serde(rename = "storageContainerUrl")]
    pub r#storage_container_url: String,
    /// The reference to the user assigned identity to use to access an Azure Container Registry instead of username and password.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolStartTaskResourceFile {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_storage_container_name",
                    &self.r#auto_storage_container_name,
                ),
                to_pulumi_object_field(
                    "blob_prefix",
                    &self.r#blob_prefix,
                ),
                to_pulumi_object_field(
                    "file_mode",
                    &self.r#file_mode,
                ),
                to_pulumi_object_field(
                    "file_path",
                    &self.r#file_path,
                ),
                to_pulumi_object_field(
                    "http_url",
                    &self.r#http_url,
                ),
                to_pulumi_object_field(
                    "storage_container_url",
                    &self.r#storage_container_url,
                ),
                to_pulumi_object_field(
                    "user_assigned_identity_id",
                    &self.r#user_assigned_identity_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolStartTaskResourceFile {
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
                    r#auto_storage_container_name: {
                        let field_value = match fields_map.get("auto_storage_container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_storage_container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#blob_prefix: {
                        let field_value = match fields_map.get("blob_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_mode: {
                        let field_value = match fields_map.get("file_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_path: {
                        let field_value = match fields_map.get("file_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_url: {
                        let field_value = match fields_map.get("http_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_container_url: {
                        let field_value = match fields_map.get("storage_container_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_container_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_assigned_identity_id: {
                        let field_value = match fields_map.get("user_assigned_identity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_assigned_identity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
