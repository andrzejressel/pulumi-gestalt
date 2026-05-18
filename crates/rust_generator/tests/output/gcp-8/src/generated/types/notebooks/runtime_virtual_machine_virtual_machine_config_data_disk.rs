#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RuntimeVirtualMachineVirtualMachineConfigDataDisk {
    /// (Output)
    /// Optional. Specifies whether the disk will be auto-deleted
    /// when the instance is deleted (but not when the disk is
    /// detached from the instance).
    #[builder(into)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Option<bool>,
    /// (Output)
    /// Optional. Indicates that this is a boot disk. The virtual
    /// machine will use the first partition of the disk for its
    /// root filesystem.
    #[builder(into)]
    #[serde(rename = "boot")]
    pub r#boot: Option<bool>,
    /// (Output)
    /// Optional. Specifies a unique device name of your choice
    /// that is reflected into the /dev/disk/by-id/google-* tree
    /// of a Linux operating system running within the instance.
    /// This name can be used to reference the device for mounting,
    /// resizing, and so on, from within the instance.
    /// If not specified, the server chooses a default device name
    /// to apply to this disk, in the form persistent-disk-x, where
    /// x is a number assigned by Google Compute Engine. This field
    /// is only applicable for persistent disks.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Option<String>,
    /// (Output)
    /// Indicates a list of features to enable on the guest operating
    /// system. Applicable only for bootable images. To see a list of
    /// available features, read `https://cloud.google.com/compute/docs/
    /// images/create-delete-deprecate-private-images#guest-os-features`
    /// options. ``
    #[builder(into)]
    #[serde(rename = "guestOsFeatures")]
    pub r#guest_os_features: Option<Vec<String>>,
    /// (Output)
    /// Output only. A zero-based index to this disk, where 0 is
    /// reserved for the boot disk. If you have many disks attached
    /// to an instance, each disk would have a unique index number.
    #[builder(into)]
    #[serde(rename = "index")]
    pub r#index: Option<i32>,
    /// Input only. Specifies the parameters for a new disk that will
    /// be created alongside the new instance. Use initialization
    /// parameters to create boot disks or local SSDs attached to the
    /// new instance. This property is mutually exclusive with the
    /// source property; you can only define one or the other, but not
    /// both.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Option<Box<super::super::types::notebooks::RuntimeVirtualMachineVirtualMachineConfigDataDiskInitializeParams>>,
    /// "Specifies the disk interface to use for attaching this disk,
    /// which is either SCSI or NVME. The default is SCSI. Persistent
    /// disks must always use SCSI and the request will fail if you attempt
    /// to attach a persistent disk in any other format than SCSI. Local SSDs
    /// can use either NVME or SCSI. For performance characteristics of SCSI
    /// over NVMe, see Local SSD performance. Valid values: * NVME * SCSI".
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Option<String>,
    /// (Output)
    /// Type of the resource. Always compute#attachedDisk for attached
    /// disks.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Option<String>,
    /// (Output)
    /// Output only. Any valid publicly visible licenses.
    #[builder(into)]
    #[serde(rename = "licenses")]
    pub r#licenses: Option<Vec<String>>,
    /// The mode in which to attach this disk, either READ_WRITE
    /// or READ_ONLY. If not specified, the default is to attach
    /// the disk in READ_WRITE mode.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// Specifies a valid partial or full URL to an existing
    /// Persistent Disk resource.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// Specifies the type of the disk, either SCRATCH or PERSISTENT.
    /// If not specified, the default is PERSISTENT.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RuntimeVirtualMachineVirtualMachineConfigDataDisk {
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
                    "auto_delete",
                    &self.r#auto_delete,
                ),
                to_pulumi_object_field(
                    "boot",
                    &self.r#boot,
                ),
                to_pulumi_object_field(
                    "device_name",
                    &self.r#device_name,
                ),
                to_pulumi_object_field(
                    "guest_os_features",
                    &self.r#guest_os_features,
                ),
                to_pulumi_object_field(
                    "index",
                    &self.r#index,
                ),
                to_pulumi_object_field(
                    "initialize_params",
                    &self.r#initialize_params,
                ),
                to_pulumi_object_field(
                    "interface",
                    &self.r#interface,
                ),
                to_pulumi_object_field(
                    "kind",
                    &self.r#kind,
                ),
                to_pulumi_object_field(
                    "licenses",
                    &self.r#licenses,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "source",
                    &self.r#source,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RuntimeVirtualMachineVirtualMachineConfigDataDisk {
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
                    r#auto_delete: {
                        let field_value = match fields_map.get("auto_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot: {
                        let field_value = match fields_map.get("boot") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_name: {
                        let field_value = match fields_map.get("device_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guest_os_features: {
                        let field_value = match fields_map.get("guest_os_features") {
                            Some(value) => value,
                            None => bail!("Missing field 'guest_os_features' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#index: {
                        let field_value = match fields_map.get("index") {
                            Some(value) => value,
                            None => bail!("Missing field 'index' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initialize_params: {
                        let field_value = match fields_map.get("initialize_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'initialize_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interface: {
                        let field_value = match fields_map.get("interface") {
                            Some(value) => value,
                            None => bail!("Missing field 'interface' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kind: {
                        let field_value = match fields_map.get("kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#licenses: {
                        let field_value = match fields_map.get("licenses") {
                            Some(value) => value,
                            None => bail!("Missing field 'licenses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source: {
                        let field_value = match fields_map.get("source") {
                            Some(value) => value,
                            None => bail!("Missing field 'source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
