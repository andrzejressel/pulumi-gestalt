#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetAssociatedEntity {
    /// Optional. Information specifying Anthos clusters as associated entities.
    #[builder(into)]
    #[serde(rename = "anthosClusters")]
    pub r#anthos_clusters: Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntityAnthosCluster>>,
    /// The name for the key in the map for which this object is mapped to in the API
    #[builder(into)]
    #[serde(rename = "entityId")]
    pub r#entity_id: String,
    /// Optional. Information specifying GKE clusters as associated entities.
    #[builder(into)]
    #[serde(rename = "gkeClusters")]
    pub r#gke_clusters: Option<Vec<super::super::types::clouddeploy::TargetAssociatedEntityGkeCluster>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TargetAssociatedEntity {
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
                "anthos_clusters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#anthos_clusters,
                )
                .await,
            );
            map.insert(
                "entity_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#entity_id,
                )
                .await,
            );
            map.insert(
                "gke_clusters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke_clusters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TargetAssociatedEntity {
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
                    r#anthos_clusters: {
                        let field_value = match fields_map.get("anthos_clusters") {
                            Some(value) => value,
                            None => bail!("Missing field 'anthos_clusters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_id: {
                        let field_value = match fields_map.get("entity_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke_clusters: {
                        let field_value = match fields_map.get("gke_clusters") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke_clusters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
