#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ImageOsDisk {
    /// Specifies the URI in Azure storage of the blob that you want to use to create the image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: Option<String>,
    /// Specifies the caching mode as `ReadWrite`, `ReadOnly`, or `None`. The default is `None`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Option<String>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this disk. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// Specifies the ID of the managed disk resource that you want to use to create the image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Option<String>,
    /// Specifies the state of the operating system contained in the blob. Currently, the only value is Generalized. Possible values are `Generalized` and `Specialized`.
    #[builder(into)]
    #[serde(rename = "osState")]
    pub r#os_state: Option<String>,
    /// Specifies the type of operating system contained in the virtual machine image. Possible values are: `Windows` or `Linux`.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: Option<String>,
    /// Specifies the size of the image to be created. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Option<i32>,
    /// The type of Storage Disk to use. Possible values are `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ImageOsDisk {
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
                    "blob_uri",
                    &self.r#blob_uri,
                ),
                to_pulumi_object_field(
                    "caching",
                    &self.r#caching,
                ),
                to_pulumi_object_field(
                    "disk_encryption_set_id",
                    &self.r#disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "managed_disk_id",
                    &self.r#managed_disk_id,
                ),
                to_pulumi_object_field(
                    "os_state",
                    &self.r#os_state,
                ),
                to_pulumi_object_field(
                    "os_type",
                    &self.r#os_type,
                ),
                to_pulumi_object_field(
                    "size_gb",
                    &self.r#size_gb,
                ),
                to_pulumi_object_field(
                    "storage_type",
                    &self.r#storage_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ImageOsDisk {
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
                    r#disk_encryption_set_id: {
                        let field_value = match fields_map.get("disk_encryption_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_encryption_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#os_state: {
                        let field_value = match fields_map.get("os_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_type: {
                        let field_value = match fields_map.get("os_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
