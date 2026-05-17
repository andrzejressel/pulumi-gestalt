#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkstationConfigHostGceInstance {
    /// An accelerator card attached to the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceAccelerator>>,
    /// A list of the boost configurations that workstations created using this workstation configuration are allowed to use.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "boostConfigs")]
    pub r#boost_configs: Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceBoostConfig>>,
    /// Size of the boot disk in GB.
    #[builder(into)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Option<i32>,
    /// A set of Compute Engine Confidential VM instance options.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "confidentialInstanceConfig")]
    pub r#confidential_instance_config: Option<Box<super::super::types::workstations::WorkstationConfigHostGceInstanceConfidentialInstanceConfig>>,
    /// Whether instances have no public IP address.
    #[builder(into)]
    #[serde(rename = "disablePublicIpAddresses")]
    pub r#disable_public_ip_addresses: Option<bool>,
    /// Whether to disable SSH access to the VM.
    #[builder(into)]
    #[serde(rename = "disableSsh")]
    pub r#disable_ssh: Option<bool>,
    /// Whether to enable nested virtualization on the Compute Engine VMs backing the Workstations.
    /// See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
    #[builder(into)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Option<bool>,
    /// The name of a Compute Engine machine type.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// Number of instances to pool for faster workstation startup.
    #[builder(into)]
    #[serde(rename = "poolSize")]
    pub r#pool_size: Option<i32>,
    /// Email address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Option<String>,
    /// Scopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have `iam.serviceAccounts.actAs` on the service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Option<Vec<String>>,
    /// A set of Compute Engine Shielded instance options.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Option<Box<super::super::types::workstations::WorkstationConfigHostGceInstanceShieldedInstanceConfig>>,
    /// Network tags to add to the Compute Engine machines backing the Workstations.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<String>>,
    /// Resource manager tags to be bound to the VM instances backing the Workstations.
    /// Tag keys and values have the same definition as
    /// https://cloud.google.com/resource-manager/docs/tags/tags-overview
    /// Keys must be in the format `tagKeys/{tag_key_id}`, and
    /// values are in the format `tagValues/456`.
    #[builder(into)]
    #[serde(rename = "vmTags")]
    pub r#vm_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkstationConfigHostGceInstance {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "accelerators".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accelerators,
                )
                .await,
            );
            map.insert(
                "boost_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boost_configs,
                )
                .await,
            );
            map.insert(
                "boot_disk_size_gb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#boot_disk_size_gb,
                )
                .await,
            );
            map.insert(
                "confidential_instance_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confidential_instance_config,
                )
                .await,
            );
            map.insert(
                "disable_public_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_public_ip_addresses,
                )
                .await,
            );
            map.insert(
                "disable_ssh".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#disable_ssh,
                )
                .await,
            );
            map.insert(
                "enable_nested_virtualization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_nested_virtualization,
                )
                .await,
            );
            map.insert(
                "machine_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#machine_type,
                )
                .await,
            );
            map.insert(
                "pool_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pool_size,
                )
                .await,
            );
            map.insert(
                "service_account".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account,
                )
                .await,
            );
            map.insert(
                "service_account_scopes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_scopes,
                )
                .await,
            );
            map.insert(
                "shielded_instance_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#shielded_instance_config,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
                )
                .await,
            );
            map.insert(
                "vm_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_tags,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("boost_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'boost_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#boot_disk_size_gb: {
                        let field_value = match fields_map.get("boot_disk_size_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'boot_disk_size_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#confidential_instance_config: {
                        let field_value = match fields_map.get("confidential_instance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidential_instance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_public_ip_addresses: {
                        let field_value = match fields_map.get("disable_public_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_public_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_ssh: {
                        let field_value = match fields_map.get("disable_ssh") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_ssh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_nested_virtualization: {
                        let field_value = match fields_map.get("enable_nested_virtualization") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_nested_virtualization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#machine_type: {
                        let field_value = match fields_map.get("machine_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'machine_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pool_size: {
                        let field_value = match fields_map.get("pool_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'pool_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account: {
                        let field_value = match fields_map.get("service_account") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_scopes: {
                        let field_value = match fields_map.get("service_account_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shielded_instance_config: {
                        let field_value = match fields_map.get("shielded_instance_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'shielded_instance_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("vm_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
