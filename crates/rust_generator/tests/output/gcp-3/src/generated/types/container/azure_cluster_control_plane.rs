#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AzureClusterControlPlane {
    /// Optional. Configuration related to application-layer secrets encryption.
    #[builder(into)]
    pub r#database_encryption: Option<Box<super::super::types::container::AzureClusterControlPlaneDatabaseEncryption>>,
    /// Optional. Configuration related to the main volume provisioned for each control plane replica. The main volume is in charge of storing all of the cluster's etcd state. When unspecified, it defaults to a 8-GiB Azure Disk.
    #[builder(into)]
    pub r#main_volume: Option<Box<super::super::types::container::AzureClusterControlPlaneMainVolume>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into)]
    pub r#proxy_config: Option<Box<super::super::types::container::AzureClusterControlPlaneProxyConfig>>,
    /// Configuration for where to place the control plane replicas. Up to three replica placement instances can be specified. If replica_placements is set, the replica placement instances will be applied to the three control plane replicas as evenly as possible.
    #[builder(into)]
    pub r#replica_placements: Option<Vec<super::super::types::container::AzureClusterControlPlaneReplicaPlacement>>,
    /// Optional. Configuration related to the root volume provisioned for each control plane replica. When unspecified, it defaults to 32-GiB Azure Disk.
    #[builder(into)]
    pub r#root_volume: Option<Box<super::super::types::container::AzureClusterControlPlaneRootVolume>>,
    /// SSH configuration for how to access the underlying control plane machines.
    #[builder(into)]
    pub r#ssh_config: Box<super::super::types::container::AzureClusterControlPlaneSshConfig>,
    /// The ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`.
    #[builder(into)]
    pub r#subnet_id: String,
    /// Optional. A set of tags to apply to all underlying control plane Azure resources.
    #[builder(into)]
    pub r#tags: Option<std::collections::BTreeMap<String, String>>,
    /// The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig.
    #[builder(into)]
    pub r#version: String,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`.
    #[builder(into)]
    pub r#vm_size: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AzureClusterControlPlane {
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
                    "databaseEncryption",
                    &self.r#database_encryption,
                ),
                to_pulumi_object_field(
                    "mainVolume",
                    &self.r#main_volume,
                ),
                to_pulumi_object_field(
                    "proxyConfig",
                    &self.r#proxy_config,
                ),
                to_pulumi_object_field(
                    "replicaPlacements",
                    &self.r#replica_placements,
                ),
                to_pulumi_object_field(
                    "rootVolume",
                    &self.r#root_volume,
                ),
                to_pulumi_object_field(
                    "sshConfig",
                    &self.r#ssh_config,
                ),
                to_pulumi_object_field(
                    "subnetId",
                    &self.r#subnet_id,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
                to_pulumi_object_field(
                    "vmSize",
                    &self.r#vm_size,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("databaseEncryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'databaseEncryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main_volume: {
                        let field_value = match fields_map.get("mainVolume") {
                            Some(value) => value,
                            None => bail!("Missing field 'mainVolume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_config: {
                        let field_value = match fields_map.get("proxyConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxyConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_placements: {
                        let field_value = match fields_map.get("replicaPlacements") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicaPlacements' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_volume: {
                        let field_value = match fields_map.get("rootVolume") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootVolume' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ssh_config: {
                        let field_value = match fields_map.get("sshConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'sshConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("vmSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'vmSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
