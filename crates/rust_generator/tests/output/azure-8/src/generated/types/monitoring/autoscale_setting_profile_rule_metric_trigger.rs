#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AutoscaleSettingProfileRuleMetricTrigger {
    /// One or more `dimensions` block as defined below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::AutoscaleSettingProfileRuleMetricTriggerDimension>>,
    /// Whether to enable metric divide by instance count.
    #[builder(into)]
    #[serde(rename = "divideByInstanceCount")]
    pub r#divide_by_instance_count: Option<bool>,
    /// The name of the metric that defines what the rule monitors, such as `Percentage CPU` for `Virtual Machine Scale Sets` and `CpuPercentage` for `App Service Plan`.
    /// 
    /// > **NOTE:** The allowed value of `metric_name` highly depends on the targeting resource type, please visit [Supported metrics with Azure Monitor](https://docs.microsoft.com/azure/azure-monitor/platform/metrics-supported) for more details.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The namespace of the metric that defines what the rule monitors, such as `microsoft.compute/virtualmachinescalesets` for `Virtual Machine Scale Sets`.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Option<String>,
    /// The ID of the Resource which the Rule monitors.
    #[builder(into)]
    #[serde(rename = "metricResourceId")]
    pub r#metric_resource_id: String,
    /// Specifies the operator used to compare the metric data and threshold. Possible values are: `Equals`, `NotEquals`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan`, `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Specifies how the metrics from multiple instances are combined. Possible values are `Average`, `Max`, `Min` and `Sum`.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: String,
    /// Specifies the threshold of the metric that triggers the scale action.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: f64,
    /// Specifies how the data that's collected should be combined over time. Possible values include `Average`, `Count`, `Maximum`, `Minimum`, `Last` and `Total`.
    #[builder(into)]
    #[serde(rename = "timeAggregation")]
    pub r#time_aggregation: String,
    /// Specifies the granularity of metrics that the rule monitors, which must be one of the pre-defined values returned from the metric definitions for the metric. This value must be between 1 minute and 12 hours an be formatted as an ISO 8601 string.
    #[builder(into)]
    #[serde(rename = "timeGrain")]
    pub r#time_grain: String,
    /// Specifies the time range for which data is collected, which must be greater than the delay in metric collection (which varies from resource to resource). This value must be between 5 minutes and 12 hours and be formatted as an ISO 8601 string.
    #[builder(into)]
    #[serde(rename = "timeWindow")]
    pub r#time_window: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AutoscaleSettingProfileRuleMetricTrigger {
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
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "divide_by_instance_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#divide_by_instance_count,
                )
                .await,
            );
            map.insert(
                "metric_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_name,
                )
                .await,
            );
            map.insert(
                "metric_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_namespace,
                )
                .await,
            );
            map.insert(
                "metric_resource_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_resource_id,
                )
                .await,
            );
            map.insert(
                "operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operator,
                )
                .await,
            );
            map.insert(
                "statistic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statistic,
                )
                .await,
            );
            map.insert(
                "threshold".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threshold,
                )
                .await,
            );
            map.insert(
                "time_aggregation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_aggregation,
                )
                .await,
            );
            map.insert(
                "time_grain".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_grain,
                )
                .await,
            );
            map.insert(
                "time_window".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_window,
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
                        let field_value = match fields_map.get("divide_by_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'divide_by_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_name: {
                        let field_value = match fields_map.get("metric_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_namespace: {
                        let field_value = match fields_map.get("metric_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_resource_id: {
                        let field_value = match fields_map.get("metric_resource_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_resource_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("time_aggregation") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_aggregation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_grain: {
                        let field_value = match fields_map.get("time_grain") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_grain' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_window: {
                        let field_value = match fields_map.get("time_window") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_window' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
