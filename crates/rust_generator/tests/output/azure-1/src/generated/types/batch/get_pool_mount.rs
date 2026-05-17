#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPoolMount {
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
                "azure_blob_file_systems".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_blob_file_systems,
                )
                .await,
            );
            map.insert(
                "azure_file_shares".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_file_shares,
                )
                .await,
            );
            map.insert(
                "cifs_mounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cifs_mounts,
                )
                .await,
            );
            map.insert(
                "nfs_mounts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfs_mounts,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPoolMount {
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
                    r#azure_blob_file_systems: {
                        let field_value = match fields_map.get("azure_blob_file_systems") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_blob_file_systems' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_file_shares: {
                        let field_value = match fields_map.get("azure_file_shares") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_file_shares' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cifs_mounts: {
                        let field_value = match fields_map.get("cifs_mounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'cifs_mounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs_mounts: {
                        let field_value = match fields_map.get("nfs_mounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs_mounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
