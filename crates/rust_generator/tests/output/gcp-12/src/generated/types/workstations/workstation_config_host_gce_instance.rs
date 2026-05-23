#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationConfigHostGceInstance {
    /// An accelerator card attached to the instance.
    /// Structure is documented below.
    #[builder(into)]
    pub r#accelerators: Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceAccelerator>>,
    /// A list of the boost configurations that workstations created using this workstation configuration are allowed to use.
    /// Structure is documented below.
    #[builder(into)]
    pub r#boost_configs: Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceBoostConfig>>,
    /// Size of the boot disk in GB.
    #[builder(into)]
    pub r#boot_disk_size_gb: Option<i32>,
    /// A set of Compute Engine Confidential VM instance options.
    /// Structure is documented below.
    #[builder(into)]
    pub r#confidential_instance_config: Option<Box<super::super::types::workstations::WorkstationConfigHostGceInstanceConfidentialInstanceConfig>>,
    /// Whether instances have no public IP address.
    #[builder(into)]
    pub r#disable_public_ip_addresses: Option<bool>,
    /// Whether to disable SSH access to the VM.
    #[builder(into)]
    pub r#disable_ssh: Option<bool>,
    /// Whether to enable nested virtualization on the Compute Engine VMs backing the Workstations.
    /// See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
    #[builder(into)]
    pub r#enable_nested_virtualization: Option<bool>,
    /// The name of a Compute Engine machine type.
    #[builder(into)]
    pub r#machine_type: Option<String>,
    /// Number of instances to pool for faster workstation startup.
    #[builder(into)]
    pub r#pool_size: Option<i32>,
    /// Email address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible.
    #[builder(into)]
    pub r#service_account: Option<String>,
    /// Scopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have `iam.serviceAccounts.actAs` on the service account.
    #[builder(into)]
    pub r#service_account_scopes: Option<Vec<String>>,
    /// A set of Compute Engine Shielded instance options.
    /// Structure is documented below.
    #[builder(into)]
    pub r#shielded_instance_config: Option<Box<super::super::types::workstations::WorkstationConfigHostGceInstanceShieldedInstanceConfig>>,
    /// Network tags to add to the Compute Engine machines backing the Workstations.
    #[builder(into)]
    pub r#tags: Option<Vec<String>>,
    /// Resource manager tags to be bound to the VM instances backing the Workstations.
    /// Tag keys and values have the same definition as
    /// https://cloud.google.com/resource-manager/docs/tags/tags-overview
    /// Keys must be in the format `tagKeys/{tag_key_id}`, and
    /// values are in the format `tagValues/456`.
    #[builder(into)]
    pub r#vm_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkstationConfigHostGceInstance {
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
                    "accelerators",
                    &self.r#accelerators,
                ),
                to_pulumi_object_field(
                    "boostConfigs",
                    &self.r#boost_configs,
                ),
                to_pulumi_object_field(
                    "bootDiskSizeGb",
                    &self.r#boot_disk_size_gb,
                ),
                to_pulumi_object_field(
                    "confidentialInstanceConfig",
                    &self.r#confidential_instance_config,
                ),
                to_pulumi_object_field(
                    "disablePublicIpAddresses",
                    &self.r#disable_public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "disableSsh",
                    &self.r#disable_ssh,
                ),
                to_pulumi_object_field(
                    "enableNestedVirtualization",
                    &self.r#enable_nested_virtualization,
                ),
                to_pulumi_object_field(
                    "machineType",
                    &self.r#machine_type,
                ),
                to_pulumi_object_field(
                    "poolSize",
                    &self.r#pool_size,
                ),
                to_pulumi_object_field(
                    "serviceAccount",
                    &self.r#service_account,
                ),
                to_pulumi_object_field(
                    "serviceAccountScopes",
                    &self.r#service_account_scopes,
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
                    "vmTags",
                    &self.r#vm_tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkstationConfigHostGceInstance {
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
                    r#accelerators: {
                        let field_value = match fields_map.get("accelerators") {
                            Some(value) => value,
                            None => bail!("Missing field 'accelerators' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boost_configs: {
                        let field_value = match fields_map.get("boostConfigs") {
                            Some(value) => value,
                            None => bail!("Missing field 'boostConfigs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk_size_gb: {
                        let field_value = match fields_map.get("bootDiskSizeGb") {
                            Some(value) => value,
                            None => bail!("Missing field 'bootDiskSizeGb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confidential_instance_config: {
                        let field_value = match fields_map.get("confidentialInstanceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidentialInstanceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_public_ip_addresses: {
                        let field_value = match fields_map.get("disablePublicIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'disablePublicIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_ssh: {
                        let field_value = match fields_map.get("disableSsh") {
                            Some(value) => value,
                            None => bail!("Missing field 'disableSsh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_nested_virtualization: {
                        let field_value = match fields_map.get("enableNestedVirtualization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableNestedVirtualization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#pool_size: {
                        let field_value = match fields_map.get("poolSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'poolSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("serviceAccount") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_scopes: {
                        let field_value = match fields_map.get("serviceAccountScopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountScopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vm_tags: {
                        let field_value = match fields_map.get("vmTags") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmTags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
