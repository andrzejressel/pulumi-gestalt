#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AzureClusterControlPlane {
    /// Optional. Configuration related to application-layer secrets encryption.
    #[builder(into)]
    #[serde(rename = "databaseEncryption")]
    pub r#database_encryption: Option<Box<super::super::types::container::AzureClusterControlPlaneDatabaseEncryption>>,
    /// Optional. Configuration related to the main volume provisioned for each control plane replica. The main volume is in charge of storing all of the cluster's etcd state. When unspecified, it defaults to a 8-GiB Azure Disk.
    #[builder(into)]
    #[serde(rename = "mainVolume")]
    pub r#main_volume: Option<Box<super::super::types::container::AzureClusterControlPlaneMainVolume>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Option<Box<super::super::types::container::AzureClusterControlPlaneProxyConfig>>,
    /// Configuration for where to place the control plane replicas. Up to three replica placement instances can be specified. If replica_placements is set, the replica placement instances will be applied to the three control plane replicas as evenly as possible.
    #[builder(into)]
    #[serde(rename = "replicaPlacements")]
    pub r#replica_placements: Option<Vec<super::super::types::container::AzureClusterControlPlaneReplicaPlacement>>,
    /// Optional. Configuration related to the root volume provisioned for each control plane replica. When unspecified, it defaults to 32-GiB Azure Disk.
    #[builder(into)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Option<Box<super::super::types::container::AzureClusterControlPlaneRootVolume>>,
    /// SSH configuration for how to access the underlying control plane machines.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<super::super::types::container::AzureClusterControlPlaneSshConfig>,
    /// The ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// Optional. A set of tags to apply to all underlying control plane Azure resources.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AzureClusterControlPlane {
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
                "database_encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#database_encryption,
                )
                .await,
            );
            map.insert(
                "main_volume".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#main_volume,
                )
                .await,
            );
            map.insert(
                "proxy_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proxy_config,
                )
                .await,
            );
            map.insert(
                "replica_placements".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replica_placements,
                )
                .await,
            );
            map.insert(
                "root_volume".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_volume,
                )
                .await,
            );
            map.insert(
                "ssh_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ssh_config,
                )
                .await,
            );
            map.insert(
                "subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_id,
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
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
                )
                .await,
            );
            map.insert(
                "vm_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_size,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AzureClusterControlPlane {
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
                    r#database_encryption: {
                        let field_value = match fields_map.get("database_encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_volume: {
                        let field_value = match fields_map.get("main_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_config: {
                        let field_value = match fields_map.get("proxy_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_placements: {
                        let field_value = match fields_map.get("replica_placements") {
                            Some(value) => value,
                            None => bail!("Missing field 'replica_placements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_volume: {
                        let field_value = match fields_map.get("root_volume") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_volume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_config: {
                        let field_value = match fields_map.get("ssh_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ssh_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_size: {
                        let field_value = match fields_map.get("vm_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
