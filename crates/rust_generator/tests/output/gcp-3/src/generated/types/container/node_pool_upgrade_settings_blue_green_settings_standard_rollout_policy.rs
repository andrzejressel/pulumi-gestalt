#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchNodeCount")]
    pub r#batch_node_count: Option<i32>,
    /// Percentage of the blue pool nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchPercentage")]
    pub r#batch_percentage: Option<f64>,
    /// Soak time after each batch gets drained.
    #[builder(into)]
    #[serde(rename = "batchSoakDuration")]
    pub r#batch_soak_duration: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy {
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
                "batch_node_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_node_count,
                )
                .await,
            );
            map.insert(
                "batch_percentage".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_percentage,
                )
                .await,
            );
            map.insert(
                "batch_soak_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_soak_duration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NodePoolUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy {
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
                    r#batch_node_count: {
                        let field_value = match fields_map.get("batch_node_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_node_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch_percentage: {
                        let field_value = match fields_map.get("batch_percentage") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_percentage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch_soak_duration: {
                        let field_value = match fields_map.get("batch_soak_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_soak_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
