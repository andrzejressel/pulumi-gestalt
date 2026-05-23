#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineStorageOsDisk {
    /// Specifies the caching requirements for the OS Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    pub r#caching: Option<String>,
    /// Specifies how the OS Disk should be created. Possible values are `Attach` (managed disks only) and `FromImage`.
    #[builder(into)]
    pub r#create_option: String,
    /// Specifies the size of the OS Disk in gigabytes.
    #[builder(into)]
    pub r#disk_size_gb: Option<i32>,
    /// Specifies the Image URI in the format `publisherName:offer:skus:version`. This field can also specify the [VHD URI](https://docs.microsoft.com/azure/virtual-machines/linux/tutorial-custom-images) of a custom VM image to clone. When cloning a Custom (Unmanaged) Disk Image the `os_type` field must be set.
    #[builder(into)]
    pub r#image_uri: Option<String>,
    /// Specifies the ID of an existing Managed Disk which should be attached as the OS Disk of this Virtual Machine. If this is set then the `create_option` must be set to `Attach`. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#managed_disk_id: Option<String>,
    /// Specifies the type of Managed Disk which should be created. Possible values are `Standard_LRS`, `StandardSSD_LRS` or `Premium_LRS`.
    /// 
    /// The following properties apply when using Unmanaged Disks:
    #[builder(into)]
    pub r#managed_disk_type: Option<String>,
    /// Specifies the name of the OS Disk.
    #[builder(into)]
    pub r#name: String,
    /// Specifies the Operating System on the OS Disk. Possible values are `Linux` and `Windows`.
    #[builder(into)]
    pub r#os_type: Option<String>,
    /// Specifies the URI of the VHD file backing this Unmanaged OS Disk. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#vhd_uri: Option<String>,
    /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
    /// 
    /// The following properties apply when using Managed Disks:
    #[builder(into)]
    pub r#write_accelerator_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineStorageOsDisk {
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
                    "diskSizeGb",
                    &self.r#disk_size_gb,
                ),
                to_pulumi_object_field(
                    "imageUri",
                    &self.r#image_uri,
                ),
                to_pulumi_object_field(
                    "managedDiskId",
                    &self.r#managed_disk_id,
                ),
                to_pulumi_object_field(
                    "managedDiskType",
                    &self.r#managed_disk_type,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "osType",
                    &self.r#os_type,
                ),
                to_pulumi_object_field(
                    "vhdUri",
                    &self.r#vhd_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineStorageOsDisk {
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
                    r#disk_size_gb: {
                        let field_value = match fields_map.get("diskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'diskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#image_uri: {
                        let field_value = match fields_map.get("imageUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'imageUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk_id: {
                        let field_value = match fields_map.get("managedDiskId") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedDiskId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_disk_type: {
                        let field_value = match fields_map.get("managedDiskType") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedDiskType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_type: {
                        let field_value = match fields_map.get("osType") {
                            Some(value) => value,
                            None => bail!("Missing field 'osType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vhd_uri: {
                        let field_value = match fields_map.get("vhdUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'vhdUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
