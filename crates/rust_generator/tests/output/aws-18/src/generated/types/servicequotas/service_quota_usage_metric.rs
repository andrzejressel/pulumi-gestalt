#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceQuotaUsageMetric {
    /// The metric dimensions.
    #[builder(into)]
    pub r#metric_dimensions: Option<Vec<super::super::types::servicequotas::ServiceQuotaUsageMetricMetricDimension>>,
    /// The name of the metric.
    #[builder(into)]
    pub r#metric_name: Option<String>,
    /// The namespace of the metric.
    #[builder(into)]
    pub r#metric_namespace: Option<String>,
    /// The metric statistic that AWS recommend you use when determining quota usage.
    #[builder(into)]
    pub r#metric_statistic_recommendation: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceQuotaUsageMetric {
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
                    "metricDimensions",
                    &self.r#metric_dimensions,
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
                    "metricStatisticRecommendation",
                    &self.r#metric_statistic_recommendation,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceQuotaUsageMetric {
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
                        let field_value = match fields_map.get("metricDimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricDimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#metric_statistic_recommendation: {
                        let field_value = match fields_map.get("metricStatisticRecommendation") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricStatisticRecommendation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
