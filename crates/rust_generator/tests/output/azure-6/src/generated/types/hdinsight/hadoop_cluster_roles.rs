#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HadoopClusterRoles {
    /// A `edge_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "edgeNode")]
    pub r#edge_node: Option<Box<super::super::types::hdinsight::HadoopClusterRolesEdgeNode>>,
    /// A `head_node` block as defined above.
    #[builder(into)]
    #[serde(rename = "headNode")]
    pub r#head_node: Box<super::super::types::hdinsight::HadoopClusterRolesHeadNode>,
    /// A `worker_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "workerNode")]
    pub r#worker_node: Box<super::super::types::hdinsight::HadoopClusterRolesWorkerNode>,
    /// A `zookeeper_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "zookeeperNode")]
    pub r#zookeeper_node: Box<super::super::types::hdinsight::HadoopClusterRolesZookeeperNode>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HadoopClusterRoles {
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
                "edge_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#edge_node,
                )
                .await,
            );
            map.insert(
                "head_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#head_node,
                )
                .await,
            );
            map.insert(
                "worker_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#worker_node,
                )
                .await,
            );
            map.insert(
                "zookeeper_node".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zookeeper_node,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HadoopClusterRoles {
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
                    r#edge_node: {
                        let field_value = match fields_map.get("edge_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#head_node: {
                        let field_value = match fields_map.get("head_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'head_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_node: {
                        let field_value = match fields_map.get("worker_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zookeeper_node: {
                        let field_value = match fields_map.get("zookeeper_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'zookeeper_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
