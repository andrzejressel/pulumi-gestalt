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
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aggregation",
                    &self.r#aggregation,
                ),
                to_pulumi_object_field(
                    "alert_sensitivity",
                    &self.r#alert_sensitivity,
                ),
                to_pulumi_object_field(
                    "dimensions",
                    &self.r#dimensions,
                ),
                to_pulumi_object_field(
                    "evaluation_failure_count",
                    &self.r#evaluation_failure_count,
                ),
                to_pulumi_object_field(
                    "evaluation_total_count",
                    &self.r#evaluation_total_count,
                ),
                to_pulumi_object_field(
                    "ignore_data_before",
                    &self.r#ignore_data_before,
                ),
                to_pulumi_object_field(
                    "metric_name",
                    &self.r#metric_name,
                ),
                to_pulumi_object_field(
                    "metric_namespace",
                    &self.r#metric_namespace,
                ),
                to_pulumi_object_field(
                    "operator",
                    &self.r#operator,
                ),
                to_pulumi_object_field(
                    "skip_metric_validation",
                    &self.r#skip_metric_validation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
