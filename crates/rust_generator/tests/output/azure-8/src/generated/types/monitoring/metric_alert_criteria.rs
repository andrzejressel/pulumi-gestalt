#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricAlertCriteria {
    /// The statistic that runs over the metric values. Possible values are `Average`, `Count`, `Minimum`, `Maximum` and `Total`.
    #[builder(into)]
    #[serde(rename = "aggregation")]
    pub r#aggregation: String,
    /// One or more `dimension` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::MetricAlertCriteriaDimension>>,
    /// One of the metric names to be monitored.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// One of the metric namespaces to be monitored.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// The criteria operator. Possible values are `Equals`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan` and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// Skip the metric validation to allow creating an alert rule on a custom metric that isn't yet emitted? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "skipMetricValidation")]
    pub r#skip_metric_validation: Option<bool>,
    /// The criteria threshold value that activates the alert.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricAlertCriteria {
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
                    "dimensions",
                    &self.r#dimensions,
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
                to_pulumi_object_field(
                    "threshold",
                    &self.r#threshold,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricAlertCriteria {
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
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#threshold: {
                        let field_value = match fields_map.get("threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
