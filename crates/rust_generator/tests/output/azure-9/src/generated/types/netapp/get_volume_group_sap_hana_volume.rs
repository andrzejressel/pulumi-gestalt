#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVolumeGroupSapHanaVolume {
    /// The ID of the Capacity Pool.
    #[builder(into)]
    #[serde(rename = "capacityPoolId")]
    pub r#capacity_pool_id: String,
    /// A `data_protection_replication` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataProtectionReplications")]
    pub r#data_protection_replications: Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeDataProtectionReplication>,
    /// A `data_protection_snapshot_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataProtectionSnapshotPolicies")]
    pub r#data_protection_snapshot_policies: Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeDataProtectionSnapshotPolicy>,
    /// A `export_policy_rule` block as defined below.
    #[builder(into)]
    #[serde(rename = "exportPolicyRules")]
    pub r#export_policy_rules: Vec<super::super::types::netapp::GetVolumeGroupSapHanaVolumeExportPolicyRule>,
    /// Volume ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// A `mount_ip_addresses` block as defined below.
    #[builder(into)]
    #[serde(rename = "mountIpAddresses")]
    pub r#mount_ip_addresses: Vec<String>,
    /// The name of this Application Volume Group for SAP HANA application.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A `protocols` block as defined below.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Vec<String>,
    /// The ID of the proximity placement group.
    #[builder(into)]
    #[serde(rename = "proximityPlacementGroupId")]
    pub r#proximity_placement_group_id: String,
    /// Volume security style.
    #[builder(into)]
    #[serde(rename = "securityStyle")]
    pub r#security_style: String,
    /// The target performance of the file system.
    #[builder(into)]
    #[serde(rename = "serviceLevel")]
    pub r#service_level: String,
    /// Is the .snapshot (NFS clients) path of a volume visible?
    #[builder(into)]
    #[serde(rename = "snapshotDirectoryVisible")]
    pub r#snapshot_directory_visible: bool,
    /// The maximum Storage Quota allowed for a file system in Gigabytes.
    #[builder(into)]
    #[serde(rename = "storageQuotaInGb")]
    pub r#storage_quota_in_gb: i32,
    /// The ID of the Subnet the NetApp Volume resides in.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// A mapping of tags assigned to the Application Volume Group.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
    /// Throughput of this volume in Mibps.
    #[builder(into)]
    #[serde(rename = "throughputInMibps")]
    pub r#throughput_in_mibps: f64,
    /// A unique file path for the volume.
    #[builder(into)]
    #[serde(rename = "volumePath")]
    pub r#volume_path: String,
    /// Volume spec name.
    #[builder(into)]
    #[serde(rename = "volumeSpecName")]
    pub r#volume_spec_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVolumeGroupSapHanaVolume {
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
                    "capacity_pool_id",
                    &self.r#capacity_pool_id,
                ),
                to_pulumi_object_field(
                    "data_protection_replications",
                    &self.r#data_protection_replications,
                ),
                to_pulumi_object_field(
                    "data_protection_snapshot_policies",
                    &self.r#data_protection_snapshot_policies,
                ),
                to_pulumi_object_field(
                    "export_policy_rules",
                    &self.r#export_policy_rules,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "mount_ip_addresses",
                    &self.r#mount_ip_addresses,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "protocols",
                    &self.r#protocols,
                ),
                to_pulumi_object_field(
                    "proximity_placement_group_id",
                    &self.r#proximity_placement_group_id,
                ),
                to_pulumi_object_field(
                    "security_style",
                    &self.r#security_style,
                ),
                to_pulumi_object_field(
                    "service_level",
                    &self.r#service_level,
                ),
                to_pulumi_object_field(
                    "snapshot_directory_visible",
                    &self.r#snapshot_directory_visible,
                ),
                to_pulumi_object_field(
                    "storage_quota_in_gb",
                    &self.r#storage_quota_in_gb,
                ),
                to_pulumi_object_field(
                    "subnet_id",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "throughput_in_mibps",
                    &self.r#throughput_in_mibps,
                ),
                to_pulumi_object_field(
                    "volume_path",
                    &self.r#volume_path,
                ),
                to_pulumi_object_field(
                    "volume_spec_name",
                    &self.r#volume_spec_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVolumeGroupSapHanaVolume {
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
                    r#capacity_pool_id: {
                        let field_value = match fields_map.get("capacity_pool_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity_pool_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_protection_replications: {
                        let field_value = match fields_map.get("data_protection_replications") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_protection_replications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_protection_snapshot_policies: {
                        let field_value = match fields_map.get("data_protection_snapshot_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_protection_snapshot_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#export_policy_rules: {
                        let field_value = match fields_map.get("export_policy_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'export_policy_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mount_ip_addresses: {
                        let field_value = match fields_map.get("mount_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'mount_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proximity_placement_group_id: {
                        let field_value = match fields_map.get("proximity_placement_group_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'proximity_placement_group_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_style: {
                        let field_value = match fields_map.get("security_style") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_style' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_level: {
                        let field_value = match fields_map.get("service_level") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_level' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshot_directory_visible: {
                        let field_value = match fields_map.get("snapshot_directory_visible") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshot_directory_visible' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_quota_in_gb: {
                        let field_value = match fields_map.get("storage_quota_in_gb") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_quota_in_gb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#throughput_in_mibps: {
                        let field_value = match fields_map.get("throughput_in_mibps") {
                            Some(value) => value,
                            None => bail!("Missing field 'throughput_in_mibps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_path: {
                        let field_value = match fields_map.get("volume_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#volume_spec_name: {
                        let field_value = match fields_map.get("volume_spec_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'volume_spec_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
