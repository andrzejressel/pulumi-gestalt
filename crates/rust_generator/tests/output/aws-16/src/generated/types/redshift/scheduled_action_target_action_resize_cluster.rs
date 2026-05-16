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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("classic".to_string(), self.r#classic.to_pulumi_value().await);
            map.insert("cluster_identifier".to_string(), self.r#cluster_identifier.to_pulumi_value().await);
            map.insert("cluster_type".to_string(), self.r#cluster_type.to_pulumi_value().await);
            map.insert("node_type".to_string(), self.r#node_type.to_pulumi_value().await);
            map.insert("number_of_nodes".to_string(), self.r#number_of_nodes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduledActionTargetActionResizeCluster {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#classic: {
                        let field_value = match fields_map.get("classic") {
                            Some(value) => value,
                            None => bail!("Missing field 'classic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cluster_identifier: {
                        let field_value = match fields_map.get("cluster_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cluster_type: {
                        let field_value = match fields_map.get("cluster_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#node_type: {
                        let field_value = match fields_map.get("node_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#number_of_nodes: {
                        let field_value = match fields_map.get("number_of_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
