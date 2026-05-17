#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfig {
    /// The configuration for running the Dataproc cluster on GKE.
    #[builder(into)]
    #[serde(rename = "gkeClusterConfig")]
    pub r#gke_cluster_config: Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfig>,
    /// A namespace within the Kubernetes cluster to deploy into. 
    /// If this namespace does not exist, it is created.
    /// If it  exists, Dataproc verifies that another Dataproc VirtualCluster is not installed into it.
    /// If not specified, the name of the Dataproc Cluster is used.
    #[builder(into)]
    #[serde(rename = "kubernetesNamespace")]
    pub r#kubernetes_namespace: Option<String>,
    /// The software configuration for this Dataproc cluster running on Kubernetes.
    #[builder(into)]
    #[serde(rename = "kubernetesSoftwareConfig")]
    pub r#kubernetes_software_config: Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigKubernetesSoftwareConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfig {
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
                "gke_cluster_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke_cluster_config,
                )
                .await,
            );
            map.insert(
                "kubernetes_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kubernetes_namespace,
                )
                .await,
            );
            map.insert(
                "kubernetes_software_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kubernetes_software_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfig {
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
                    r#gke_cluster_config: {
                        let field_value = match fields_map.get("gke_cluster_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_cluster_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubernetes_namespace: {
                        let field_value = match fields_map.get("kubernetes_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubernetes_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kubernetes_software_config: {
                        let field_value = match fields_map.get("kubernetes_software_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'kubernetes_software_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
