#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig {
    /// A target GKE cluster to deploy to. It must be in the same project and region as the Dataproc cluster 
    /// (the GKE cluster can be zonal or regional)
    #[builder(into)]
    #[serde(rename = "gkeClusterTarget")]
    pub r#gke_cluster_target: Option<String>,
    /// GKE node pools where workloads will be scheduled. At least one node pool must be assigned the `DEFAULT` 
    /// GkeNodePoolTarget.Role. If a GkeNodePoolTarget is not specified, Dataproc constructs a `DEFAULT` GkeNodePoolTarget.
    /// Each role can be given to only one GkeNodePoolTarget. All node pools must have the same location settings.
    #[builder(into)]
    #[serde(rename = "nodePoolTargets")]
    pub r#node_pool_targets: Option<Vec<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig {
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
                "gke_cluster_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke_cluster_target,
                )
                .await,
            );
            map.insert(
                "node_pool_targets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_pool_targets,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig {
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
                    r#gke_cluster_target: {
                        let field_value = match fields_map.get("gke_cluster_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_cluster_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_pool_targets: {
                        let field_value = match fields_map.get("node_pool_targets") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_pool_targets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
