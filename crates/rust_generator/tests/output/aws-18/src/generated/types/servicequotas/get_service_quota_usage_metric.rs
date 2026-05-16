#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceQuotaUsageMetric {
    /// The metric dimensions.
    #[builder(into)]
    #[serde(rename = "metricDimensions")]
    pub r#metric_dimensions: Vec<super::super::types::servicequotas::GetServiceQuotaUsageMetricMetricDimension>,
    /// The name of the metric.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The namespace of the metric.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// The metric statistic that AWS recommend you use when determining quota usage.
    #[builder(into)]
    #[serde(rename = "metricStatisticRecommendation")]
    pub r#metric_statistic_recommendation: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceQuotaUsageMetric {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("metric_dimensions".to_string(), self.r#metric_dimensions.to_pulumi_value().await);
            map.insert("metric_name".to_string(), self.r#metric_name.to_pulumi_value().await);
            map.insert("metric_namespace".to_string(), self.r#metric_namespace.to_pulumi_value().await);
            map.insert("metric_statistic_recommendation".to_string(), self.r#metric_statistic_recommendation.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceQuotaUsageMetric {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#metric_dimensions: {
                        let field_value = match fields_map.get("metric_dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::servicequotas::GetServiceQuotaUsageMetricMetricDimension> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metric_name: {
                        let field_value = match fields_map.get("metric_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metric_namespace: {
                        let field_value = match fields_map.get("metric_namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metric_statistic_recommendation: {
                        let field_value = match fields_map.get("metric_statistic_recommendation") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_statistic_recommendation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
