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
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "account_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_key,
                )
                .await,
            );
            map.insert(
                "account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_name,
                )
                .await,
            );
            map.insert(
                "blobfuse_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blobfuse_options,
                )
                .await,
            );
            map.insert(
                "container_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_name,
                )
                .await,
            );
            map.insert(
                "identity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identity_id,
                )
                .await,
            );
            map.insert(
                "relative_mount_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#relative_mount_path,
                )
                .await,
            );
            map.insert(
                "sas_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sas_key,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("account_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account_name: {
                        let field_value = match fields_map.get("account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#blobfuse_options: {
                        let field_value = match fields_map.get("blobfuse_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'blobfuse_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_name: {
                        let field_value = match fields_map.get("container_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_id: {
                        let field_value = match fields_map.get("identity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#relative_mount_path: {
                        let field_value = match fields_map.get("relative_mount_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'relative_mount_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sas_key: {
                        let field_value = match fields_map.get("sas_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'sas_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
