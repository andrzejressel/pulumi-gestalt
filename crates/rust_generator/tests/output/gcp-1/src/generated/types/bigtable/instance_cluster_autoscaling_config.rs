#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceClusterAutoscalingConfig {
    /// The target CPU utilization for autoscaling, in percentage. Must be between 10 and 80.
    #[builder(into)]
    #[serde(rename = "cpuTarget")]
    pub r#cpu_target: i32,
    /// The maximum number of nodes for autoscaling.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: i32,
    /// The minimum number of nodes for autoscaling.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: i32,
    /// The target storage utilization for autoscaling, in GB, for each node in a cluster. This number is limited between 2560 (2.5TiB) and 5120 (5TiB) for a SSD cluster and between 8192 (8TiB) and 16384 (16 TiB) for an HDD cluster. If not set, whatever is already set for the cluster will not change, or if the cluster is just being created, it will use the default value of 2560 for SSD clusters and 8192 for HDD clusters.
    /// 
    /// !> **Warning**: Only one of `autoscaling_config` or `num_nodes` should be set for a cluster. If both are set, `num_nodes` is ignored. If none is set, autoscaling will be disabled and sized to the current node count.
    #[builder(into)]
    #[serde(rename = "storageTarget")]
    pub r#storage_target: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceClusterAutoscalingConfig {
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
                "cpu_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_target,
                )
                .await,
            );
            map.insert(
                "max_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_nodes,
                )
                .await,
            );
            map.insert(
                "min_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_nodes,
                )
                .await,
            );
            map.insert(
                "storage_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_target,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceClusterAutoscalingConfig {
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
                    r#cpu_target: {
                        let field_value = match fields_map.get("cpu_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_nodes: {
                        let field_value = match fields_map.get("max_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_nodes: {
                        let field_value = match fields_map.get("min_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_target: {
                        let field_value = match fields_map.get("storage_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
