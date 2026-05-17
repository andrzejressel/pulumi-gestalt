#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxClusterAutoScalingConfiguration {
    /// Metric your cluster will track in order to scale in and out. For example, CPU_UTILIZATION_PERCENTAGE is the average CPU usage across all nodes in a cluster.
    #[builder(into)]
    #[serde(rename = "autoScalingMetric")]
    pub r#auto_scaling_metric: String,
    /// Highest number of nodes to scale. Cannot be greater than 5
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: i32,
    /// Desired value of chosen `auto_scaling_metric`. When metric drops below this value, cluster will scale in. When metric goes above this value, cluster will scale out. Can be set between 0 and 100 percent.
    #[builder(into)]
    #[serde(rename = "metricTarget")]
    pub r#metric_target: f64,
    /// Lowest number of nodes to scale. Must be at least 1 and less than the `max_node_count`. If nodes in cluster belong to multiple availability zones, then `min_node_count` must be at least 3.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: i32,
    /// Duration in seconds that FinSpace will wait after a scale in event before initiating another scaling event.
    #[builder(into)]
    #[serde(rename = "scaleInCooldownSeconds")]
    pub r#scale_in_cooldown_seconds: f64,
    /// Duration in seconds that FinSpace will wait after a scale out event before initiating another scaling event.
    #[builder(into)]
    #[serde(rename = "scaleOutCooldownSeconds")]
    pub r#scale_out_cooldown_seconds: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KxClusterAutoScalingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_scaling_metric",
                    &self.r#auto_scaling_metric,
                ),
                to_pulumi_object_field(
                    "max_node_count",
                    &self.r#max_node_count,
                ),
                to_pulumi_object_field(
                    "metric_target",
                    &self.r#metric_target,
                ),
                to_pulumi_object_field(
                    "min_node_count",
                    &self.r#min_node_count,
                ),
                to_pulumi_object_field(
                    "scale_in_cooldown_seconds",
                    &self.r#scale_in_cooldown_seconds,
                ),
                to_pulumi_object_field(
                    "scale_out_cooldown_seconds",
                    &self.r#scale_out_cooldown_seconds,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KxClusterAutoScalingConfiguration {
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
                    r#auto_scaling_metric: {
                        let field_value = match fields_map.get("auto_scaling_metric") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_scaling_metric' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#metric_target: {
                        let field_value = match fields_map.get("metric_target") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scale_in_cooldown_seconds: {
                        let field_value = match fields_map.get("scale_in_cooldown_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_in_cooldown_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_out_cooldown_seconds: {
                        let field_value = match fields_map.get("scale_out_cooldown_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_out_cooldown_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
