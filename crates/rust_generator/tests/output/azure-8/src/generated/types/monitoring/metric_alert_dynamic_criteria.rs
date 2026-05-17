#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricAlertDynamicCriteria {
    /// The statistic that runs over the metric values. Possible values are `Average`, `Count`, `Minimum`, `Maximum` and `Total`.
    #[builder(into)]
    #[serde(rename = "aggregation")]
    pub r#aggregation: String,
    /// The extent of deviation required to trigger an alert. Possible values are `Low`, `Medium` and `High`.
    #[builder(into)]
    #[serde(rename = "alertSensitivity")]
    pub r#alert_sensitivity: String,
    /// One or more `dimension` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::MetricAlertDynamicCriteriaDimension>>,
    /// The number of violations to trigger an alert. Should be smaller or equal to `evaluation_total_count`. Defaults to `4`.
    #[builder(into)]
    #[serde(rename = "evaluationFailureCount")]
    pub r#evaluation_failure_count: Option<i32>,
    /// The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (`window_size`) and the selected number of aggregated points. Defaults to `4`.
    #[builder(into)]
    #[serde(rename = "evaluationTotalCount")]
    pub r#evaluation_total_count: Option<i32>,
    /// The [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) date from which to start learning the metric historical data and calculate the dynamic thresholds.
    #[builder(into)]
    #[serde(rename = "ignoreDataBefore")]
    pub r#ignore_data_before: Option<String>,
    /// One of the metric names to be monitored.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// One of the metric namespaces to be monitored.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// The criteria operator. Possible values are `LessThan`, `GreaterThan` and `GreaterOrLessThan`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Skip the metric validation to allow creating an alert rule on a custom metric that isn't yet emitted?
    #[builder(into)]
    #[serde(rename = "skipMetricValidation")]
    pub r#skip_metric_validation: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricAlertDynamicCriteria {
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
                "aggregation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregation,
                )
                .await,
            );
            map.insert(
                "alert_sensitivity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#alert_sensitivity,
                )
                .await,
            );
            map.insert(
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "evaluation_failure_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_failure_count,
                )
                .await,
            );
            map.insert(
                "evaluation_total_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_total_count,
                )
                .await,
            );
            map.insert(
                "ignore_data_before".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ignore_data_before,
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
                "operator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#operator,
                )
                .await,
            );
            map.insert(
                "skip_metric_validation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#skip_metric_validation,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricAlertDynamicCriteria {
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
                    r#aggregation: {
                        let field_value = match fields_map.get("aggregation") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alert_sensitivity: {
                        let field_value = match fields_map.get("alert_sensitivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_sensitivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_failure_count: {
                        let field_value = match fields_map.get("evaluation_failure_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_failure_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_total_count: {
                        let field_value = match fields_map.get("evaluation_total_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_total_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_data_before: {
                        let field_value = match fields_map.get("ignore_data_before") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_data_before' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_metric_validation: {
                        let field_value = match fields_map.get("skip_metric_validation") {
                            Some(value) => value,
                            None => bail!("Missing field 'skip_metric_validation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
