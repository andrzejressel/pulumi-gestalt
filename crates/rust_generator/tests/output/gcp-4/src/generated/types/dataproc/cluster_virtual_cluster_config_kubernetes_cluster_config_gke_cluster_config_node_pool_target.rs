#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget {
    /// The target GKE node pool.
    #[builder(into)]
    #[serde(rename = "nodePool")]
    pub r#node_pool: String,
    /// The configuration for the GKE node pool. 
    /// If specified, Dataproc attempts to create a node pool with the specified shape.
    /// If one with the same name already exists, it is verified against all specified fields.
    /// If a field differs, the virtual cluster creation will fail.
    #[builder(into)]
    #[serde(rename = "nodePoolConfig")]
    pub r#node_pool_config: Option<Box<super::super::types::dataproc::ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfig>>,
    /// The roles associated with the GKE node pool. 
    /// One of `"DEFAULT"`, `"CONTROLLER"`, `"SPARK_DRIVER"` or `"SPARK_EXECUTOR"`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget {
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
                "node_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_pool,
                )
                .await,
            );
            map.insert(
                "node_pool_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_pool_config,
                )
                .await,
            );
            map.insert(
                "roles".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#roles,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTarget {
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
                    r#node_pool: {
                        let field_value = match fields_map.get("node_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_pool_config: {
                        let field_value = match fields_map.get("node_pool_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_pool_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#roles: {
                        let field_value = match fields_map.get("roles") {
                            Some(value) => value,
                            None => bail!("Missing field 'roles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
