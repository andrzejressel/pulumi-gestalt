#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy {
    #[builder(into)]
    #[serde(rename = "autoscalePolicyId")]
    pub r#autoscale_policy_id: String,
    /// Utilization thresholds pertaining to amount of consumed memory.
    #[builder(into)]
    #[serde(rename = "consumedMemoryThresholds")]
    pub r#consumed_memory_thresholds: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyConsumedMemoryThreshold>,
    /// Utilization thresholds pertaining to CPU utilization.
    #[builder(into)]
    #[serde(rename = "cpuThresholds")]
    pub r#cpu_thresholds: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyCpuThreshold>,
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
    pub r#storage_thresholds: Vec<super::super::types::vmwareengine::GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicyStorageThreshold>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "autoscale_policy_id",
                    &self.r#autoscale_policy_id,
                ),
                to_pulumi_object_field(
                    "consumed_memory_thresholds",
                    &self.r#consumed_memory_thresholds,
                ),
                to_pulumi_object_field(
                    "cpu_thresholds",
                    &self.r#cpu_thresholds,
                ),
                to_pulumi_object_field(
                    "node_type_id",
                    &self.r#node_type_id,
                ),
                to_pulumi_object_field(
                    "scale_out_size",
                    &self.r#scale_out_size,
                ),
                to_pulumi_object_field(
                    "storage_thresholds",
                    &self.r#storage_thresholds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPrivateCloudManagementClusterAutoscalingSettingAutoscalingPolicy {
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
