#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PhpAppLayerLoadBasedAutoScalingDownscaling {
    #[builder(into)]
    #[serde(rename = "alarms")]
    pub r#alarms: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "cpuThreshold")]
    pub r#cpu_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "ignoreMetricsTime")]
    pub r#ignore_metrics_time: Option<i32>,
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Option<i32>,
    #[builder(into)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "memoryThreshold")]
    pub r#memory_threshold: Option<f64>,
    #[builder(into)]
    #[serde(rename = "thresholdsWaitTime")]
    pub r#thresholds_wait_time: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PhpAppLayerLoadBasedAutoScalingDownscaling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "alarms",
                    &self.r#alarms,
                ),
                to_pulumi_object_field(
                    "cpu_threshold",
                    &self.r#cpu_threshold,
                ),
                to_pulumi_object_field(
                    "ignore_metrics_time",
                    &self.r#ignore_metrics_time,
                ),
                to_pulumi_object_field(
                    "instance_count",
                    &self.r#instance_count,
                ),
                to_pulumi_object_field(
                    "load_threshold",
                    &self.r#load_threshold,
                ),
                to_pulumi_object_field(
                    "memory_threshold",
                    &self.r#memory_threshold,
                ),
                to_pulumi_object_field(
                    "thresholds_wait_time",
                    &self.r#thresholds_wait_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PhpAppLayerLoadBasedAutoScalingDownscaling {
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
                    r#alarms: {
                        let field_value = match fields_map.get("alarms") {
                            Some(value) => value,
                            None => bail!("Missing field 'alarms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_threshold: {
                        let field_value = match fields_map.get("cpu_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_metrics_time: {
                        let field_value = match fields_map.get("ignore_metrics_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_metrics_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_count: {
                        let field_value = match fields_map.get("instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_threshold: {
                        let field_value = match fields_map.get("load_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_threshold: {
                        let field_value = match fields_map.get("memory_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#thresholds_wait_time: {
                        let field_value = match fields_map.get("thresholds_wait_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'thresholds_wait_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
