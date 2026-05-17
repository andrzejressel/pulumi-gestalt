#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig {
    /// The number of local SSD disks to attach to the node, 
    /// which is limited by the maximum number of disks allowable per zone.
    #[builder(into)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Option<i32>,
    /// The name of a Compute Engine machine type.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Option<String>,
    /// Minimum CPU platform to be used by this instance. 
    /// The instance may be scheduled on the specified or a newer CPU platform.
    /// Specify the friendly names of CPU platforms, such as "Intel Haswell" or "Intel Sandy Bridge".
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Option<String>,
    /// Whether the nodes are created as preemptible VM instances. 
    /// Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the
    /// CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role).
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Option<bool>,
    /// Spot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag.
    #[builder(into)]
    #[serde(rename = "spot")]
    pub r#spot: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig {
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
                "local_ssd_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_count,
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
                "min_cpu_platform".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_cpu_platform,
                )
                .await,
            );
            map.insert(
                "preemptible".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemptible,
                )
                .await,
            );
            map.insert(
                "spot".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spot,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig {
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
                    r#local_ssd_count: {
                        let field_value = match fields_map.get("local_ssd_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#min_cpu_platform: {
                        let field_value = match fields_map.get("min_cpu_platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_cpu_platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptible: {
                        let field_value = match fields_map.get("preemptible") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptible' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spot: {
                        let field_value = match fields_map.get("spot") {
                            Some(value) => value,
                            None => bail!("Missing field 'spot' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
