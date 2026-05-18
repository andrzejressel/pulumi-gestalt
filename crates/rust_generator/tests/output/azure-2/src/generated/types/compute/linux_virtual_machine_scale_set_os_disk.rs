#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxVirtualMachineScaleSetOsDisk {
    /// The Type of Caching which should be used for the Internal OS Disk. Possible values are `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: String,
    /// A `diff_disk_settings` block as defined above. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diffDiskSettings")]
    pub r#diff_disk_settings: Option<Box<super::super::types::compute::LinuxVirtualMachineScaleSetOsDiskDiffDiskSettings>>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this OS Disk. Conflicts with `secure_vm_disk_encryption_set_id`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
    /// 
    /// > **Note:** Disk Encryption Sets are in Public Preview in a limited set of regions
    #[builder(into)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Option<String>,
    /// The Size of the Internal OS Disk in GB, if you wish to vary from the size used in the image this Virtual Machine Scale Set is sourced from.
    /// 
    /// > **Note:** If specified this must be equal to or larger than the size of the Image the VM Scale Set is based on. When creating a larger disk than exists in the image you'll need to repartition the disk to use the remaining space.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Option<i32>,
    /// The ID of the Disk Encryption Set which should be used to Encrypt the OS Disk when the Virtual Machine Scale Set is Confidential VMSS. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** `secure_vm_disk_encryption_set_id` can only be specified when `security_encryption_type` is set to `DiskWithVMGuestState`.
    #[builder(into)]
    #[serde(rename = "secureVmDiskEncryptionSetId")]
    pub r#secure_vm_disk_encryption_set_id: Option<String>,
    /// Encryption Type when the Virtual Machine Scale Set is Confidential VMSS. Possible values are `VMGuestStateOnly` and `DiskWithVMGuestState`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** `vtpm_enabled` must be set to `true` when `security_encryption_type` is specified.
    /// 
    /// > **Note:** `encryption_at_host_enabled` cannot be set to `true` when `security_encryption_type` is set to `DiskWithVMGuestState`.
    #[builder(into)]
    #[serde(rename = "securityEncryptionType")]
    pub r#security_encryption_type: Option<String>,
    /// The Type of Storage Account which should back this the Internal OS Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS` and `Premium_ZRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: String,
    /// Should Write Accelerator be Enabled for this OS Disk? Defaults to `false`.
    /// 
    /// > **Note:** This requires that the `storage_account_type` is set to `Premium_LRS` and that `caching` is set to `None`.
    #[builder(into)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxVirtualMachineScaleSetOsDisk {
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
                    "caching",
                    &self.r#caching,
                ),
                to_pulumi_object_field(
                    "diff_disk_settings",
                    &self.r#diff_disk_settings,
                ),
                to_pulumi_object_field(
                    "disk_encryption_set_id",
                    &self.r#disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "disk_size_gb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "secure_vm_disk_encryption_set_id",
                    &self.r#secure_vm_disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "security_encryption_type",
                    &self.r#security_encryption_type,
                ),
                to_pulumi_object_field(
                    "storage_account_type",
                    &self.r#storage_account_type,
                ),
                to_pulumi_object_field(
                    "write_accelerator_enabled",
                    &self.r#write_accelerator_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxVirtualMachineScaleSetOsDisk {
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
                    r#secure_vm_disk_encryption_set_id: {
                        let field_value = match fields_map.get("secure_vm_disk_encryption_set_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'secure_vm_disk_encryption_set_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_encryption_type: {
                        let field_value = match fields_map.get("security_encryption_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_encryption_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
