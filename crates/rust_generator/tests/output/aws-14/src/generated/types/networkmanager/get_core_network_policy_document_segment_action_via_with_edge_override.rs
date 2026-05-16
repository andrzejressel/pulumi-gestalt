#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride {
    /// A list of a list of strings. The list of edges associated with the network function group.
    #[builder(into)]
    #[serde(rename = "edgeSets")]
    pub r#edge_sets: Option<Vec<Vec<String>>>,
    /// The preferred edge to use.
    #[builder(into)]
    #[serde(rename = "useEdge")]
    pub r#use_edge: Option<String>,
    /// The preferred edge to use.
    #[builder(into)]
    #[serde(rename = "useEdgeLocation")]
    pub r#use_edge_location: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("edge_sets".to_string(), self.r#edge_sets.to_pulumi_value().await);
            map.insert("use_edge".to_string(), self.r#use_edge.to_pulumi_value().await);
            map.insert("use_edge_location".to_string(), self.r#use_edge_location.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride {
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
                    r#edge_sets: {
                        let field_value = match fields_map.get("edge_sets") {
                            Some(value) => value,
                            None => bail!("Missing field 'edge_sets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<Vec<String>>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#use_edge: {
                        let field_value = match fields_map.get("use_edge") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_edge' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#use_edge_location: {
                        let field_value = match fields_map.get("use_edge_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_edge_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
