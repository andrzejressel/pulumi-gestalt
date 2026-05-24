#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceGceSetup {
    /// The hardware accelerators used on this instance. If you use accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list).
    /// Currently supports only one accelerator configuration.
    /// Structure is documented below.
    #[builder(into)]
    pub r#accelerator_configs: Option<Vec<super::super::types::workbench::InstanceGceSetupAcceleratorConfig>>,
    /// The definition of a boot disk.
    /// Structure is documented below.
    #[builder(into)]
    pub r#boot_disk: Option<Box<super::super::types::workbench::InstanceGceSetupBootDisk>>,
    /// Use a container image to start the workbench instance.
    /// Structure is documented below.
    #[builder(into)]
    pub r#container_image: Option<Box<super::super::types::workbench::InstanceGceSetupContainerImage>>,
    /// Data disks attached to the VM instance. Currently supports only one data disk.
    /// Structure is documented below.
    #[builder(into)]
    pub r#data_disks: Option<Box<super::super::types::workbench::InstanceGceSetupDataDisks>>,
    /// Optional. If true, no external IP will be assigned to this VM instance.
    #[builder(into)]
    pub r#disable_public_ip: Option<bool>,
    /// Optional. Flag to enable ip forwarding or not, default false/off.
    /// https://cloud.google.com/vpc/docs/using-routes#canipforward
    #[builder(into)]
    pub r#enable_ip_forwarding: Option<bool>,
    /// Optional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// Optional. Custom metadata to apply to this instance.
    #[builder(into)]
    pub r#metadata: Option<std::collections::BTreeMap<String, String>>,
    /// The network interfaces for the VM. Supports only one interface.
    /// Structure is documented below.
    #[builder(into)]
    pub r#network_interfaces: Option<Vec<super::super::types::workbench::InstanceGceSetupNetworkInterface>>,
    /// The service account that serves as an identity for the VM instance. Currently supports only one service account.
    /// Structure is documented below.
    #[builder(into)]
    pub r#service_accounts: Option<Vec<super::super::types::workbench::InstanceGceSetupServiceAccount>>,
    /// A set of Shielded Instance options. See [Images using supported Shielded
    /// VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm).
    /// Not all combinations are valid.
    /// Structure is documented below.
    #[builder(into)]
    pub r#shielded_instance_config: Option<Box<super::super::types::workbench::InstanceGceSetupShieldedInstanceConfig>>,
    /// Optional. The Compute Engine tags to add to instance (see [Tagging
    /// instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
    #[builder(into)]
    pub r#tags: Option<Vec<String>>,
    /// Definition of a custom Compute Engine virtual machine image for starting
    /// a workbench instance with the environment installed directly on the VM.
    /// Structure is documented below.
    #[builder(into)]
    pub r#vm_image: Option<Box<super::super::types::workbench::InstanceGceSetupVmImage>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceGceSetup {
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
                    "acceleratorConfigs",
                    &self.r#accelerator_configs,
                ),
                to_pulumi_object_field(
                    "bootDisk",
                    &self.r#boot_disk,
                ),
                to_pulumi_object_field(
                    "containerImage",
                    &self.r#container_image,
                ),
                to_pulumi_object_field(
                    "dataDisks",
                    &self.r#data_disks,
                ),
                to_pulumi_object_field(
                    "disablePublicIp",
                    &self.r#disable_public_ip,
                ),
                to_pulumi_object_field(
                    "enableIpForwarding",
                    &self.r#enable_ip_forwarding,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "metadata",
                    &self.r#metadata,
                ),
                to_pulumi_object_field(
                    "networkInterfaces",
                    &self.r#network_interfaces,
                ),
                to_pulumi_object_field(
                    "serviceAccounts",
                    &self.r#service_accounts,
                ),
                to_pulumi_object_field(
                    "shieldedInstanceConfig",
                    &self.r#shielded_instance_config,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "vmImage",
                    &self.r#vm_image,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceGceSetup {
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
                    r#accelerator_configs: {
                        let field_value = match fields_map.get("acceleratorConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'acceleratorConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk: {
                        let field_value = match fields_map.get("bootDisk") {
                            Some(value) => value,
                            None => bail!("Missing field 'bootDisk' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_image: {
                        let field_value = match fields_map.get("containerImage") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerImage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_disks: {
                        let field_value = match fields_map.get("dataDisks") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataDisks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_public_ip: {
                        let field_value = match fields_map.get("disablePublicIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'disablePublicIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ip_forwarding: {
                        let field_value = match fields_map.get("enableIpForwarding") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableIpForwarding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machineType") {
                            Some(value) => value,
                            None => bail!("Missing field 'machineType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata: {
                        let field_value = match fields_map.get("metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_interfaces: {
                        let field_value = match fields_map.get("networkInterfaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkInterfaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_accounts: {
                        let field_value = match fields_map.get("serviceAccounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shielded_instance_config: {
                        let field_value = match fields_map.get("shieldedInstanceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'shieldedInstanceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_image: {
                        let field_value = match fields_map.get("vmImage") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmImage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
