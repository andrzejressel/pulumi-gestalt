#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolFixedScale {
    /// It determines what to do with a node and its running task(s) if the pool size is decreasing. Values are `Requeue`, `RetainedData`, `TaskCompletion` and `Terminate`.
    #[builder(into)]
    #[serde(rename = "nodeDeallocationMethod")]
    pub r#node_deallocation_method: Option<String>,
    /// The timeout for resize operations. Defaults to `PT15M`.
    #[builder(into)]
    #[serde(rename = "resizeTimeout")]
    pub r#resize_timeout: Option<String>,
    /// The number of nodes in the Batch pool. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "targetDedicatedNodes")]
    pub r#target_dedicated_nodes: Option<i32>,
    /// The number of low priority nodes in the Batch pool. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "targetLowPriorityNodes")]
    pub r#target_low_priority_nodes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PoolFixedScale {
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
                "node_deallocation_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_deallocation_method,
                )
                .await,
            );
            map.insert(
                "resize_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resize_timeout,
                )
                .await,
            );
            map.insert(
                "target_dedicated_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_dedicated_nodes,
                )
                .await,
            );
            map.insert(
                "target_low_priority_nodes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_low_priority_nodes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PoolFixedScale {
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
                    r#node_deallocation_method: {
                        let field_value = match fields_map.get("node_deallocation_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_deallocation_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resize_timeout: {
                        let field_value = match fields_map.get("resize_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'resize_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_dedicated_nodes: {
                        let field_value = match fields_map.get("target_dedicated_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_dedicated_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_low_priority_nodes: {
                        let field_value = match fields_map.get("target_low_priority_nodes") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_low_priority_nodes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
