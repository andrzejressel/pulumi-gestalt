#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureOnlineStoreBigtableAutoScaling {
    /// A percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%.
    #[builder(into)]
    #[serde(rename = "cpuUtilizationTarget")]
    pub r#cpu_utilization_target: Option<i32>,
    /// The maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: i32,
    /// The minimum number of nodes to scale down to. Must be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AiFeatureOnlineStoreBigtableAutoScaling {
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
                "cpu_utilization_target".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_utilization_target,
                )
                .await,
            );
            map.insert(
                "max_node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_node_count,
                )
                .await,
            );
            map.insert(
                "min_node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_node_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AiFeatureOnlineStoreBigtableAutoScaling {
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
                    r#cpu_utilization_target: {
                        let field_value = match fields_map.get("cpu_utilization_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_utilization_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_node_count: {
                        let field_value = match fields_map.get("max_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_node_count: {
                        let field_value = match fields_map.get("min_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
