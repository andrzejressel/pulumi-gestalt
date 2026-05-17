#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterAutoscalingSettingAutoscalingPolicy {
    #[builder(into)]
    #[serde(rename = "autoscalePolicyId")]
    pub r#autoscale_policy_id: String,
    /// Utilization thresholds pertaining to amount of consumed memory.
    #[builder(into)]
    #[serde(rename = "consumedMemoryThresholds")]
    pub r#consumed_memory_thresholds: Vec<super::super::types::vmwareengine::GetClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold>,
    /// Utilization thresholds pertaining to CPU utilization.
    #[builder(into)]
    #[serde(rename = "cpuThresholds")]
    pub r#cpu_thresholds: Vec<super::super::types::vmwareengine::GetClusterAutoscalingSettingAutoscalingPolicyCpuThreshold>,
    /// The canonical identifier of the node type to add or remove.
    #[builder(into)]
    #[serde(rename = "nodeTypeId")]
    pub r#node_type_id: String,
    /// Number of nodes to add to a cluster during a scale-out operation.
    /// Must be divisible by 2 for stretched clusters.
    #[builder(into)]
    #[serde(rename = "scaleOutSize")]
    pub r#scale_out_size: i32,
    /// Utilization thresholds pertaining to amount of consumed storage.
    #[builder(into)]
    #[serde(rename = "storageThresholds")]
    pub r#storage_thresholds: Vec<super::super::types::vmwareengine::GetClusterAutoscalingSettingAutoscalingPolicyStorageThreshold>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterAutoscalingSettingAutoscalingPolicy {
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
                "autoscale_policy_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autoscale_policy_id,
                )
                .await,
            );
            map.insert(
                "consumed_memory_thresholds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumed_memory_thresholds,
                )
                .await,
            );
            map.insert(
                "cpu_thresholds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_thresholds,
                )
                .await,
            );
            map.insert(
                "node_type_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_type_id,
                )
                .await,
            );
            map.insert(
                "scale_out_size".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scale_out_size,
                )
                .await,
            );
            map.insert(
                "storage_thresholds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_thresholds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterAutoscalingSettingAutoscalingPolicy {
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
                    r#autoscale_policy_id: {
                        let field_value = match fields_map.get("autoscale_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoscale_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#consumed_memory_thresholds: {
                        let field_value = match fields_map.get("consumed_memory_thresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumed_memory_thresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_thresholds: {
                        let field_value = match fields_map.get("cpu_thresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_thresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_type_id: {
                        let field_value = match fields_map.get("node_type_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_type_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_out_size: {
                        let field_value = match fields_map.get("scale_out_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_out_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_thresholds: {
                        let field_value = match fields_map.get("storage_thresholds") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_thresholds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
