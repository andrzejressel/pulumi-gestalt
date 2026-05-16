#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodeProperty {
    /// Specifies the node index for the main node of a multi-node parallel job. This node index value must be fewer than the number of nodes.
    #[builder(into)]
    #[serde(rename = "mainNode")]
    pub r#main_node: i32,
    /// A list of node ranges and their properties that are associated with a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "nodeRangeProperties")]
    pub r#node_range_properties: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangeProperty>,
    /// The number of nodes that are associated with a multi-node parallel job.
    #[builder(into)]
    #[serde(rename = "numNodes")]
    pub r#num_nodes: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodeProperty {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("main_node".to_string(), self.r#main_node.to_pulumi_value().await);
            map.insert("node_range_properties".to_string(), self.r#node_range_properties.to_pulumi_value().await);
            map.insert("num_nodes".to_string(), self.r#num_nodes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodeProperty {
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
                    r#main_node: {
                        let field_value = match fields_map.get("main_node") {
                            Some(value) => value,
                            None => bail!("Missing field 'main_node' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#node_range_properties: {
                        let field_value = match fields_map.get("node_range_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_range_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangeProperty> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#num_nodes: {
                        let field_value = match fields_map.get("num_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
