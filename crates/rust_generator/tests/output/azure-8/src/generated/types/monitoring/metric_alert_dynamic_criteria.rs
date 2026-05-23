#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricAlertDynamicCriteria {
    /// The statistic that runs over the metric values. Possible values are `Average`, `Count`, `Minimum`, `Maximum` and `Total`.
    #[builder(into)]
    pub r#aggregation: String,
    /// The extent of deviation required to trigger an alert. Possible values are `Low`, `Medium` and `High`.
    #[builder(into)]
    pub r#alert_sensitivity: String,
    /// One or more `dimension` blocks as defined below.
    #[builder(into)]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::MetricAlertDynamicCriteriaDimension>>,
    /// The number of violations to trigger an alert. Should be smaller or equal to `evaluation_total_count`. Defaults to `4`.
    #[builder(into)]
    pub r#evaluation_failure_count: Option<i32>,
    /// The number of aggregated lookback points. The lookback time window is calculated based on the aggregation granularity (`window_size`) and the selected number of aggregated points. Defaults to `4`.
    #[builder(into)]
    pub r#evaluation_total_count: Option<i32>,
    /// The [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) date from which to start learning the metric historical data and calculate the dynamic thresholds.
    #[builder(into)]
    pub r#ignore_data_before: Option<String>,
    /// One of the metric names to be monitored.
    #[builder(into)]
    pub r#metric_name: String,
    /// One of the metric namespaces to be monitored.
    #[builder(into)]
    pub r#metric_namespace: String,
    /// The criteria operator. Possible values are `LessThan`, `GreaterThan` and `GreaterOrLessThan`.
    #[builder(into)]
    pub r#operator: String,
    /// Skip the metric validation to allow creating an alert rule on a custom metric that isn't yet emitted?
    #[builder(into)]
    pub r#skip_metric_validation: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricAlertDynamicCriteria {
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
                    "aggregation",
                    &self.r#aggregation,
                ),
                to_pulumi_object_field(
                    "alertSensitivity",
                    &self.r#alert_sensitivity,
                ),
                to_pulumi_object_field(
                    "dimensions",
                    &self.r#dimensions,
                ),
                to_pulumi_object_field(
                    "evaluationFailureCount",
                    &self.r#evaluation_failure_count,
                ),
                to_pulumi_object_field(
                    "evaluationTotalCount",
                    &self.r#evaluation_total_count,
                ),
                to_pulumi_object_field(
                    "ignoreDataBefore",
                    &self.r#ignore_data_before,
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
                    "operator",
                    &self.r#operator,
                ),
                to_pulumi_object_field(
                    "skipMetricValidation",
                    &self.r#skip_metric_validation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("alertSensitivity") {
                            Some(value) => value,
                            None => bail!("Missing field 'alertSensitivity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("evaluationFailureCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluationFailureCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_total_count: {
                        let field_value = match fields_map.get("evaluationTotalCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluationTotalCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_data_before: {
                        let field_value = match fields_map.get("ignoreDataBefore") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreDataBefore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#operator: {
                        let field_value = match fields_map.get("operator") {
                            Some(value) => value,
                            None => bail!("Missing field 'operator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_metric_validation: {
                        let field_value = match fields_map.get("skipMetricValidation") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipMetricValidation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
