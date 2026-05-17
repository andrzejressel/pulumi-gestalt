#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnit {
    /// Specify the full name of the Active Directory Organizational Unit container object prepared for the deployment, including the domain components. For example:`OU=HCI01,DC=contoso,DC=com`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "activeDirectoryOrganizationalUnitPath")]
    pub r#active_directory_organizational_unit_path: String,
    /// Whether to enable BitLocker for boot volume. Possible values are `true` and `false`. When set to `true`, BitLocker XTS_AES 256-bit encryption is enabled for all data-at-rest on the OS volume of your Azure Stack HCI cluster. This setting is TPM-hardware dependent. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "bitlockerBootVolumeEnabled")]
    pub r#bitlocker_boot_volume_enabled: Option<bool>,
    /// Whether to enable BitLocker for data volume. Possible values are `true` and `false`. When set to `true`, BitLocker XTS-AES 256-bit encryption is enabled for all data-at-rest on your Azure Stack HCI cluster shared volumes. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "bitlockerDataVolumeEnabled")]
    pub r#bitlocker_data_volume_enabled: Option<bool>,
    /// A `cluster` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<super::super::types::stack::HciDeploymentSettingScaleUnitCluster>,
    /// Whether to enable credential guard. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "credentialGuardEnabled")]
    pub r#credential_guard_enabled: Option<bool>,
    /// Specifies the FQDN for deploying cluster. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "domainFqdn")]
    pub r#domain_fqdn: String,
    /// Whether to enable drift control. Possible values are `true` and `false`. When set to `true`, the security baseline is re-applied regularly. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "driftControlEnabled")]
    pub r#drift_control_enabled: Option<bool>,
    /// Whether to enable DRTM protection. Possible values are `true` and `false`. When set to `true`, Secure Boot is enabled on your Azure HCI cluster. This setting is hardware dependent. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "drtmProtectionEnabled")]
    pub r#drtm_protection_enabled: Option<bool>,
    /// Whether to collect log data to facilitate quicker issue resolution. Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "episodicDataUploadEnabled")]
    pub r#episodic_data_upload_enabled: Option<bool>,
    /// Whether to store data sent to Microsoft in EU. The log and diagnostic data is sent to the appropriate diagnostics servers depending upon where your cluster resides. Setting this to `false` results in all data sent to Microsoft to be stored outside of the EU. Possible values are `true` and `false`. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "euLocationEnabled")]
    pub r#eu_location_enabled: Option<bool>,
    /// A `host_network` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "hostNetwork")]
    pub r#host_network: Box<super::super::types::stack::HciDeploymentSettingScaleUnitHostNetwork>,
    /// Whether to enable HVCI protection. Possible values are `true` and `false`. When set to `true`, Hypervisor-protected Code Integrity is enabled on your Azure HCI cluster. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "hvciProtectionEnabled")]
    pub r#hvci_protection_enabled: Option<bool>,
    /// One or more `infrastructure_network` blocks as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "infrastructureNetworks")]
    pub r#infrastructure_networks: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitInfrastructureNetwork>,
    /// Specifies the name prefix to deploy cluster. It must be 1-8 characters long and contain only letters, numbers and hyphens Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: String,
    /// A `optional_service` block as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "optionalService")]
    pub r#optional_service: Box<super::super::types::stack::HciDeploymentSettingScaleUnitOptionalService>,
    /// One or more `physical_node` blocks as defined above. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "physicalNodes")]
    pub r#physical_nodes: Vec<super::super::types::stack::HciDeploymentSettingScaleUnitPhysicalNode>,
    /// The URI to the Key Vault or secret store. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "secretsLocation")]
    pub r#secrets_location: String,
    /// Whether to enable side channel mitigation. Possible values are `true` and `false`. When set to `true`, all side channel mitigations are enabled on your Azure HCI cluster. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "sideChannelMitigationEnabled")]
    pub r#side_channel_mitigation_enabled: Option<bool>,
    /// Whether to enable SMB cluster encryption. Possible values are `true` and `false`. When set to `true`, cluster east-west traffic is encrypted. Defaults to `false`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "smbClusterEncryptionEnabled")]
    pub r#smb_cluster_encryption_enabled: Option<bool>,
    /// Whether to enable SMB signing. Possible values are `true` and `false`. When set to `true`, the SMB default instance requires sign in for the client and server services. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "smbSigningEnabled")]
    pub r#smb_signing_enabled: Option<bool>,
    /// A `storage` block as defined below. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "storage")]
    pub r#storage: Box<super::super::types::stack::HciDeploymentSettingScaleUnitStorage>,
    /// Whether the telemetry data will be sent to Microsoft. Possible values are `true` and `false`. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "streamingDataClientEnabled")]
    pub r#streaming_data_client_enabled: Option<bool>,
    /// Whether to enable WDAC. Possible values are `true` and `false`. When set to `true`, applications and the code that you can run on your Azure Stack HCI cluster are limited. Defaults to `true`. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "wdacEnabled")]
    pub r#wdac_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HciDeploymentSettingScaleUnit {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "active_directory_organizational_unit_path",
                    &self.r#active_directory_organizational_unit_path,
                ),
                to_pulumi_object_field(
                    "bitlocker_boot_volume_enabled",
                    &self.r#bitlocker_boot_volume_enabled,
                ),
                to_pulumi_object_field(
                    "bitlocker_data_volume_enabled",
                    &self.r#bitlocker_data_volume_enabled,
                ),
                to_pulumi_object_field(
                    "cluster",
                    &self.r#cluster,
                ),
                to_pulumi_object_field(
                    "credential_guard_enabled",
                    &self.r#credential_guard_enabled,
                ),
                to_pulumi_object_field(
                    "domain_fqdn",
                    &self.r#domain_fqdn,
                ),
                to_pulumi_object_field(
                    "drift_control_enabled",
                    &self.r#drift_control_enabled,
                ),
                to_pulumi_object_field(
                    "drtm_protection_enabled",
                    &self.r#drtm_protection_enabled,
                ),
                to_pulumi_object_field(
                    "episodic_data_upload_enabled",
                    &self.r#episodic_data_upload_enabled,
                ),
                to_pulumi_object_field(
                    "eu_location_enabled",
                    &self.r#eu_location_enabled,
                ),
                to_pulumi_object_field(
                    "host_network",
                    &self.r#host_network,
                ),
                to_pulumi_object_field(
                    "hvci_protection_enabled",
                    &self.r#hvci_protection_enabled,
                ),
                to_pulumi_object_field(
                    "infrastructure_networks",
                    &self.r#infrastructure_networks,
                ),
                to_pulumi_object_field(
                    "name_prefix",
                    &self.r#name_prefix,
                ),
                to_pulumi_object_field(
                    "optional_service",
                    &self.r#optional_service,
                ),
                to_pulumi_object_field(
                    "physical_nodes",
                    &self.r#physical_nodes,
                ),
                to_pulumi_object_field(
                    "secrets_location",
                    &self.r#secrets_location,
                ),
                to_pulumi_object_field(
                    "side_channel_mitigation_enabled",
                    &self.r#side_channel_mitigation_enabled,
                ),
                to_pulumi_object_field(
                    "smb_cluster_encryption_enabled",
                    &self.r#smb_cluster_encryption_enabled,
                ),
                to_pulumi_object_field(
                    "smb_signing_enabled",
                    &self.r#smb_signing_enabled,
                ),
                to_pulumi_object_field(
                    "storage",
                    &self.r#storage,
                ),
                to_pulumi_object_field(
                    "streaming_data_client_enabled",
                    &self.r#streaming_data_client_enabled,
                ),
                to_pulumi_object_field(
                    "wdac_enabled",
                    &self.r#wdac_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HciDeploymentSettingScaleUnit {
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
                    r#active_directory_organizational_unit_path: {
                        let field_value = match fields_map.get("active_directory_organizational_unit_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directory_organizational_unit_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bitlocker_boot_volume_enabled: {
                        let field_value = match fields_map.get("bitlocker_boot_volume_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitlocker_boot_volume_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bitlocker_data_volume_enabled: {
                        let field_value = match fields_map.get("bitlocker_data_volume_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'bitlocker_data_volume_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster: {
                        let field_value = match fields_map.get("cluster") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credential_guard_enabled: {
                        let field_value = match fields_map.get("credential_guard_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'credential_guard_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_fqdn: {
                        let field_value = match fields_map.get("domain_fqdn") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_fqdn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drift_control_enabled: {
                        let field_value = match fields_map.get("drift_control_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'drift_control_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#drtm_protection_enabled: {
                        let field_value = match fields_map.get("drtm_protection_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'drtm_protection_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#episodic_data_upload_enabled: {
                        let field_value = match fields_map.get("episodic_data_upload_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'episodic_data_upload_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eu_location_enabled: {
                        let field_value = match fields_map.get("eu_location_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'eu_location_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_network: {
                        let field_value = match fields_map.get("host_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hvci_protection_enabled: {
                        let field_value = match fields_map.get("hvci_protection_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'hvci_protection_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#infrastructure_networks: {
                        let field_value = match fields_map.get("infrastructure_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'infrastructure_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_prefix: {
                        let field_value = match fields_map.get("name_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#optional_service: {
                        let field_value = match fields_map.get("optional_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'optional_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#physical_nodes: {
                        let field_value = match fields_map.get("physical_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'physical_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secrets_location: {
                        let field_value = match fields_map.get("secrets_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'secrets_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#side_channel_mitigation_enabled: {
                        let field_value = match fields_map.get("side_channel_mitigation_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'side_channel_mitigation_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smb_cluster_encryption_enabled: {
                        let field_value = match fields_map.get("smb_cluster_encryption_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'smb_cluster_encryption_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smb_signing_enabled: {
                        let field_value = match fields_map.get("smb_signing_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'smb_signing_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage: {
                        let field_value = match fields_map.get("storage") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#streaming_data_client_enabled: {
                        let field_value = match fields_map.get("streaming_data_client_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'streaming_data_client_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wdac_enabled: {
                        let field_value = match fields_map.get("wdac_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'wdac_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
