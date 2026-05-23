#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingProfileRuleMetricTrigger {
    /// One or more `dimensions` block as defined below.
    #[builder(into)]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::AutoscaleSettingProfileRuleMetricTriggerDimension>>,
    /// Whether to enable metric divide by instance count.
    #[builder(into)]
    pub r#divide_by_instance_count: Option<bool>,
    /// The name of the metric that defines what the rule monitors, such as `Percentage CPU` for `Virtual Machine Scale Sets` and `CpuPercentage` for `App Service Plan`.
    /// 
    /// > **NOTE:** The allowed value of `metric_name` highly depends on the targeting resource type, please visit [Supported metrics with Azure Monitor](https://docs.microsoft.com/azure/azure-monitor/platform/metrics-supported) for more details.
    #[builder(into)]
    pub r#metric_name: String,
    /// The namespace of the metric that defines what the rule monitors, such as `microsoft.compute/virtualmachinescalesets` for `Virtual Machine Scale Sets`.
    #[builder(into)]
    pub r#metric_namespace: Option<String>,
    /// The ID of the Resource which the Rule monitors.
    #[builder(into)]
    pub r#metric_resource_id: String,
    /// Specifies the operator used to compare the metric data and threshold. Possible values are: `Equals`, `NotEquals`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan`, `LessThanOrEqual`.
    #[builder(into)]
    pub r#operator: String,
    /// Specifies how the metrics from multiple instances are combined. Possible values are `Average`, `Max`, `Min` and `Sum`.
    #[builder(into)]
    pub r#statistic: String,
    /// Specifies the threshold of the metric that triggers the scale action.
    #[builder(into)]
    pub r#threshold: f64,
    /// Specifies how the data that's collected should be combined over time. Possible values include `Average`, `Count`, `Maximum`, `Minimum`, `Last` and `Total`.
    #[builder(into)]
    pub r#time_aggregation: String,
    /// Specifies the granularity of metrics that the rule monitors, which must be one of the pre-defined values returned from the metric definitions for the metric. This value must be between 1 minute and 12 hours an be formatted as an ISO 8601 string.
    #[builder(into)]
    pub r#time_grain: String,
    /// Specifies the time range for which data is collected, which must be greater than the delay in metric collection (which varies from resource to resource). This value must be between 5 minutes and 12 hours and be formatted as an ISO 8601 string.
    #[builder(into)]
    pub r#time_window: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscaleSettingProfileRuleMetricTrigger {
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
                    "dimensions",
                    &self.r#dimensions,
                ),
                to_pulumi_object_field(
                    "divideByInstanceCount",
                    &self.r#divide_by_instance_count,
                ),
                to_pulumi_object_field(
                    "metricName",
                    &self.r#metric_name,
                ),
                to_pulumi_object_field(
                    "metricNamespace",
                    &self.r#metric_namespace,
                ),
                to_pulumi_object_field(
                    "metricResourceId",
                    &self.r#metric_resource_id,
                ),
                to_pulumi_object_field(
                    "operator",
                    &self.r#operator,
                ),
                to_pulumi_object_field(
                    "statistic",
                    &self.r#statistic,
                ),
                to_pulumi_object_field(
                    "threshold",
                    &self.r#threshold,
                ),
                to_pulumi_object_field(
                    "timeAggregation",
                    &self.r#time_aggregation,
                ),
                to_pulumi_object_field(
                    "timeGrain",
                    &self.r#time_grain,
                ),
                to_pulumi_object_field(
                    "timeWindow",
                    &self.r#time_window,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AutoscaleSettingProfileRuleMetricTrigger {
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
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#divide_by_instance_count: {
                        let field_value = match fields_map.get("divideByInstanceCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'divideByInstanceCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_name: {
                        let field_value = match fields_map.get("metricName") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_namespace: {
                        let field_value = match fields_map.get("metricNamespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricNamespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_resource_id: {
                        let field_value = match fields_map.get("metricResourceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricResourceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statistic: {
                        let field_value = match fields_map.get("statistic") {
                            Some(value) => value,
                            None => bail!("Missing field 'statistic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_aggregation: {
                        let field_value = match fields_map.get("timeAggregation") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeAggregation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_grain: {
                        let field_value = match fields_map.get("timeGrain") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeGrain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_window: {
                        let field_value = match fields_map.get("timeWindow") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeWindow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
