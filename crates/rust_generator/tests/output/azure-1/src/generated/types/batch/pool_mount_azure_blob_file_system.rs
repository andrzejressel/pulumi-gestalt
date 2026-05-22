#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolMountAzureBlobFileSystem {
    /// The Azure Storage Account key. This property is mutually exclusive with both `sas_key` and `identity_id`; exactly one must be specified.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: Option<String>,
    /// The Azure Storage Account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// Additional command line options to pass to the mount command. These are 'net use' options in Windows and 'mount' options in Linux.
    #[builder(into)]
    #[serde(rename = "blobfuseOptions")]
    pub r#blobfuse_options: Option<String>,
    /// The Azure Blob Storage Container name.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: String,
    /// The ARM resource id of the user assigned identity. This property is mutually exclusive with both `account_key` and `sas_key`; exactly one must be specified.
    #[builder(into)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Option<String>,
    /// The relative path on compute node where the file system will be mounted All file systems are mounted relative to the Batch mounts directory, accessible via the `AZ_BATCH_NODE_MOUNTS_DIR` environment variable.
    #[builder(into)]
    #[serde(rename = "relativeMountPath")]
    pub r#relative_mount_path: String,
    /// The Azure Storage SAS token. This property is mutually exclusive with both `account_key` and `identity_id`; exactly one must be specified.
    #[builder(into)]
    #[serde(rename = "sasKey")]
    pub r#sas_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolMountAzureBlobFileSystem {
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
                    "accountKey",
                    &self.r#account_key,
                ),
                to_pulumi_object_field(
                    "accountName",
                    &self.r#account_name,
                ),
                to_pulumi_object_field(
                    "blobfuseOptions",
                    &self.r#blobfuse_options,
                ),
                to_pulumi_object_field(
                    "containerName",
                    &self.r#container_name,
                ),
                to_pulumi_object_field(
                    "identityId",
                    &self.r#identity_id,
                ),
                to_pulumi_object_field(
                    "relativeMountPath",
                    &self.r#relative_mount_path,
                ),
                to_pulumi_object_field(
                    "sasKey",
                    &self.r#sas_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolMountAzureBlobFileSystem {
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
                    r#account_key: {
                        let field_value = match fields_map.get("accountKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account_name: {
                        let field_value = match fields_map.get("accountName") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#blobfuse_options: {
                        let field_value = match fields_map.get("blobfuseOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'blobfuseOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_name: {
                        let field_value = match fields_map.get("containerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_id: {
                        let field_value = match fields_map.get("identityId") {
                            Some(value) => value,
                            None => bail!("Missing field 'identityId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relative_mount_path: {
                        let field_value = match fields_map.get("relativeMountPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'relativeMountPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sas_key: {
                        let field_value = match fields_map.get("sasKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'sasKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
