#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyTargetTrackingConfigurationCustomizedMetricSpecification {
    /// Dimensions of the metric.
    #[builder(into)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetricDimension>>,
    /// Name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Option<String>,
    /// Metrics to include, as a metric data query.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Vec<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecificationMetric>>,
    /// Namespace of the metric.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// Statistic of the metric.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: Option<String>,
    /// Unit of the metric.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyTargetTrackingConfigurationCustomizedMetricSpecification {
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
                    "metric_dimensions",
                    &self.r#metric_dimensions,
                ),
                to_pulumi_object_field(
                    "metric_name",
                    &self.r#metric_name,
                ),
                to_pulumi_object_field(
                    "metrics",
                    &self.r#metrics,
                ),
                to_pulumi_object_field(
                    "namespace",
                    &self.r#namespace,
                ),
                to_pulumi_object_field(
                    "statistic",
                    &self.r#statistic,
                ),
                to_pulumi_object_field(
                    "unit",
                    &self.r#unit,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyTargetTrackingConfigurationCustomizedMetricSpecification {
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
                    r#metric_dimensions: {
                        let field_value = match fields_map.get("metric_dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#metrics: {
                        let field_value = match fields_map.get("metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespace: {
                        let field_value = match fields_map.get("namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#unit: {
                        let field_value = match fields_map.get("unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
