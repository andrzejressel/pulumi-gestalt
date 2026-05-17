#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledActionTargetActionResizeCluster {
    /// A boolean value indicating whether the resize operation is using the classic resize process. Default: `false`.
    #[builder(into)]
    #[serde(rename = "classic")]
    pub r#classic: Option<bool>,
    /// The unique identifier for the cluster to resize.
    #[builder(into)]
    #[serde(rename = "clusterIdentifier")]
    pub r#cluster_identifier: String,
    /// The new cluster type for the specified cluster.
    #[builder(into)]
    #[serde(rename = "clusterType")]
    pub r#cluster_type: Option<String>,
    /// The new node type for the nodes you are adding.
    #[builder(into)]
    #[serde(rename = "nodeType")]
    pub r#node_type: Option<String>,
    /// The new number of nodes for the cluster.
    #[builder(into)]
    #[serde(rename = "numberOfNodes")]
    pub r#number_of_nodes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduledActionTargetActionResizeCluster {
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
                "classic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classic,
                )
                .await,
            );
            map.insert(
                "cluster_identifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_identifier,
                )
                .await,
            );
            map.insert(
                "cluster_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cluster_type,
                )
                .await,
            );
            map.insert(
                "node_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type,
                )
                .await,
            );
            map.insert(
                "number_of_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_nodes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduledActionTargetActionResizeCluster {
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
                    r#classic: {
                        let field_value = match fields_map.get("classic") {
                            Some(value) => value,
                            None => bail!("Missing field 'classic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_identifier: {
                        let field_value = match fields_map.get("cluster_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_type: {
                        let field_value = match fields_map.get("cluster_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_type: {
                        let field_value = match fields_map.get("node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_nodes: {
                        let field_value = match fields_map.get("number_of_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
