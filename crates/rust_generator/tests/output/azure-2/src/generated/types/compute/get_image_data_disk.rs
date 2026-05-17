#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetImageDataDisk {
    /// the URI in Azure storage of the blob used to create the image.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: String,
    /// the caching mode for the Data Disk, such as `ReadWrite`, `ReadOnly`, or `None`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// the logical unit number of the data disk.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: i32,
    /// the ID of the Managed Disk used as the Data Disk Image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: String,
    /// the size of this Data Disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetImageDataDisk {
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
                "blob_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blob_uri,
                )
                .await,
            );
            map.insert(
                "caching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#caching,
                )
                .await,
            );
            map.insert(
                "lun".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lun,
                )
                .await,
            );
            map.insert(
                "managed_disk_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_disk_id,
                )
                .await,
            );
            map.insert(
                "size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#size_gb,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetImageDataDisk {
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
                    r#blob_uri: {
                        let field_value = match fields_map.get("blob_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#caching: {
                        let field_value = match fields_map.get("caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lun: {
                        let field_value = match fields_map.get("lun") {
                            Some(value) => value,
                            None => bail!("Missing field 'lun' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk_id: {
                        let field_value = match fields_map.get("managed_disk_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_disk_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#size_gb: {
                        let field_value = match fields_map.get("size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
