#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetOsDisk {
    /// The Type of Caching which should be used for the Internal OS Disk. Possible values are `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// A `diff_disk_settings` block as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diffDiskSettings")]
    pub r#diff_disk_settings: Option<Box<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDiskDiffDiskSettings>>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this OS Disk. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Disk Encryption Sets are in Public Preview in a limited set of regions
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// The Size of the Internal OS Disk in GB, if you wish to vary from the size used in the image this Virtual Machine Scale Set is sourced from.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Option<i32>,
    /// The Type of Storage Account which should back this the Internal OS Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS` and `Premium_ZRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: String,
    /// Specifies if Write Accelerator is enabled on the OS Disk. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetOsDisk {
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
                "caching".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#caching,
                )
                .await,
            );
            map.insert(
                "diff_disk_settings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#diff_disk_settings,
                )
                .await,
            );
            map.insert(
                "disk_encryption_set_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_encryption_set_id,
                )
                .await,
            );
            map.insert(
                "disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disk_size_gb,
                )
                .await,
            );
            map.insert(
                "storage_account_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_type,
                )
                .await,
            );
            map.insert(
                "write_accelerator_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#write_accelerator_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetOsDisk {
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
                    r#caching: {
                        let field_value = match fields_map.get("caching") {
                            Some(value) => value,
                            None => bail!("Missing field 'caching' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#diff_disk_settings: {
                        let field_value = match fields_map.get("diff_disk_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'diff_disk_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_type: {
                        let field_value = match fields_map.get("storage_account_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_accelerator_enabled: {
                        let field_value = match fields_map.get("write_accelerator_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'write_accelerator_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
