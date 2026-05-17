#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodePoolAutoConfig {
    /// Linux node configuration options.
    #[builder(into)]
    #[serde(rename = "linuxNodeConfigs")]
    pub r#linux_node_configs: Vec<super::super::types::container::GetClusterNodePoolAutoConfigLinuxNodeConfig>,
    /// Collection of Compute Engine network tags that can be applied to a node's underlying VM instance.
    #[builder(into)]
    #[serde(rename = "networkTags")]
    pub r#network_tags: Vec<super::super::types::container::GetClusterNodePoolAutoConfigNetworkTag>,
    /// Node kubelet configs.
    #[builder(into)]
    #[serde(rename = "nodeKubeletConfigs")]
    pub r#node_kubelet_configs: Vec<super::super::types::container::GetClusterNodePoolAutoConfigNodeKubeletConfig>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: std::collections::HashMap<String, String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterNodePoolAutoConfig {
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
                "linux_node_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_node_configs,
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
                "node_kubelet_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_kubelet_configs,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterNodePoolAutoConfig {
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
                    r#linux_node_configs: {
                        let field_value = match fields_map.get("linux_node_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_node_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_kubelet_configs: {
                        let field_value = match fields_map.get("node_kubelet_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_kubelet_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
