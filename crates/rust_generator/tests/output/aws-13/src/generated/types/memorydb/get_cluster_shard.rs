#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterShard {
    /// Name of the cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Set of nodes in this shard.
    #[builder(into)]
    #[serde(rename = "nodes")]
    pub r#nodes: Vec<super::super::types::memorydb::GetClusterShardNode>,
    /// Number of individual nodes in this shard.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: i32,
    /// Keyspace for this shard. Example: `0-16383`.
    #[builder(into)]
    #[serde(rename = "slots")]
    pub r#slots: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterShard {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("nodes".to_string(), self.r#nodes.to_pulumi_value().await);
            map.insert("num_nodes".to_string(), self.r#num_nodes.to_pulumi_value().await);
            map.insert("slots".to_string(), self.r#slots.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterShard {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#nodes: {
                        let field_value = match fields_map.get("nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::memorydb::GetClusterShardNode> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#num_nodes: {
                        let field_value = match fields_map.get("num_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#slots: {
                        let field_value = match fields_map.get("slots") {
                            Some(value) => value,
                            None => bail!("Missing field 'slots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
