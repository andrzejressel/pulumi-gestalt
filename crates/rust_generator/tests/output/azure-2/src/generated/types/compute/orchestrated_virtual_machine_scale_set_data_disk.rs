#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OrchestratedVirtualMachineScaleSetDataDisk {
    /// The type of Caching which should be used for this Data Disk. Possible values are None, ReadOnly and ReadWrite.
    #[builder(into)]
    pub r#caching: String,
    /// The create option which should be used for this Data Disk. Possible values are Empty and FromImage. Defaults to `Empty`. (FromImage should only be used if the source image includes data disks).
    #[builder(into)]
    pub r#create_option: Option<String>,
    /// The ID of the Disk Encryption Set which should be used to encrypt the Data Disk. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#disk_encryption_set_id: Option<String>,
    /// The size of the Data Disk which should be created. Required if `create_option` is specified as `Empty`.
    #[builder(into)]
    pub r#disk_size_gb: Option<i32>,
    /// The Logical Unit Number of the Data Disk, which must be unique within the Virtual Machine. Required if `create_option` is specified as `Empty`.
    #[builder(into)]
    pub r#lun: Option<i32>,
    /// The Type of Storage Account which should back this Data Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS` and `UltraSSD_LRS`.
    #[builder(into)]
    pub r#storage_account_type: String,
    /// Specifies the Read-Write IOPS for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into)]
    pub r#ultra_ssd_disk_iops_read_write: Option<i32>,
    /// Specifies the bandwidth in MB per second for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into)]
    pub r#ultra_ssd_disk_mbps_read_write: Option<i32>,
    /// Specifies if Write Accelerator is enabled on the Data Disk. Defaults to `false`.
    #[builder(into)]
    pub r#write_accelerator_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OrchestratedVirtualMachineScaleSetDataDisk {
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
                    "createOption",
                    &self.r#create_option,
                ),
                to_pulumi_object_field(
                    "diskEncryptionSetId",
                    &self.r#disk_encryption_set_id,
                ),
                to_pulumi_object_field(
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "lun",
                    &self.r#lun,
                ),
                to_pulumi_object_field(
                    "storageAccountType",
                    &self.r#storage_account_type,
                ),
                to_pulumi_object_field(
                    "ultraSsdDiskIopsReadWrite",
                    &self.r#ultra_ssd_disk_iops_read_write,
                ),
                to_pulumi_object_field(
                    "ultraSsdDiskMbpsReadWrite",
                    &self.r#ultra_ssd_disk_mbps_read_write,
                ),
                to_pulumi_object_field(
                    "writeAcceleratorEnabled",
                    &self.r#write_accelerator_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OrchestratedVirtualMachineScaleSetDataDisk {
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
                    r#create_option: {
                        let field_value = match fields_map.get("createOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'createOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_encryption_set_id: {
                        let field_value = match fields_map.get("diskEncryptionSetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskEncryptionSetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#storage_account_type: {
                        let field_value = match fields_map.get("storageAccountType") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageAccountType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ultra_ssd_disk_iops_read_write: {
                        let field_value = match fields_map.get("ultraSsdDiskIopsReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'ultraSsdDiskIopsReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ultra_ssd_disk_mbps_read_write: {
                        let field_value = match fields_map.get("ultraSsdDiskMbpsReadWrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'ultraSsdDiskMbpsReadWrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_accelerator_enabled: {
                        let field_value = match fields_map.get("writeAcceleratorEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'writeAcceleratorEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
