#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolAutoConfig {
    /// Linux system configuration for the cluster's automatically provisioned node pools. Only `cgroup_mode` field is supported in `node_pool_auto_config`. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "linuxNodeConfig")]
    pub r#linux_node_config: Option<Box<super::super::types::container::ClusterNodePoolAutoConfigLinuxNodeConfig>>,
    /// The network tag config for the cluster's automatically provisioned node pools. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Option<Box<super::super::types::container::ClusterNodePoolAutoConfigNetworkTags>>,
    /// Kubelet configuration for Autopilot clusters. Currently, only `insecure_kubelet_readonly_port_enabled` is supported here.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nodeKubeletConfig")]
    pub r#node_kubelet_config: Option<Box<super::super::types::container::ClusterNodePoolAutoConfigNodeKubeletConfig>>,
    /// A map of resource manager tag keys and values to be attached to the nodes for managing Compute Engine firewalls using Network Firewall Policies. Tags must be according to specifications found [here](https://cloud.google.com/vpc/docs/tags-firewalls-overview#specifications). A maximum of 5 tag key-value pairs can be specified. Existing tags will be replaced with new values. Tags must be in one of the following formats ([KEY]=[VALUE]) 1. `tagKeys/{tag_key_id}=tagValues/{tag_value_id}` 2. `{org_id}/{tag_key_name}={tag_value_name}` 3. `{project_id}/{tag_key_name}={tag_value_name}`.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodePoolAutoConfig {
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
                "linux_node_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_node_config,
                )
                .await,
            );
            map.insert(
                "network_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_tags,
                )
                .await,
            );
            map.insert(
                "node_kubelet_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_kubelet_config,
                )
                .await,
            );
            map.insert(
                "resource_manager_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_manager_tags,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodePoolAutoConfig {
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
                    r#linux_node_config: {
                        let field_value = match fields_map.get("linux_node_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_node_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_tags: {
                        let field_value = match fields_map.get("network_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_kubelet_config: {
                        let field_value = match fields_map.get("node_kubelet_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_kubelet_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_manager_tags: {
                        let field_value = match fields_map.get("resource_manager_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_manager_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
