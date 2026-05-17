#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GraphQlApiEnhancedMetricsConfig {
    /// How data source metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_DATA_SOURCE_METRICS`, `PER_DATA_SOURCE_METRICS`
    #[builder(into)]
    #[serde(rename = "dataSourceLevelMetricsBehavior")]
    pub r#data_source_level_metrics_behavior: String,
    /// How operation metrics will be emitted to CloudWatch. Valid values: `ENABLED`, `DISABLED`
    #[builder(into)]
    #[serde(rename = "operationLevelMetricsConfig")]
    pub r#operation_level_metrics_config: String,
    /// How resolver metrics will be emitted to CloudWatch. Valid values: `FULL_REQUEST_RESOLVER_METRICS`, `PER_RESOLVER_METRICS`
    #[builder(into)]
    #[serde(rename = "resolverLevelMetricsBehavior")]
    pub r#resolver_level_metrics_behavior: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GraphQlApiEnhancedMetricsConfig {
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
                    "data_source_level_metrics_behavior",
                    &self.r#data_source_level_metrics_behavior,
                ),
                to_pulumi_object_field(
                    "operation_level_metrics_config",
                    &self.r#operation_level_metrics_config,
                ),
                to_pulumi_object_field(
                    "resolver_level_metrics_behavior",
                    &self.r#resolver_level_metrics_behavior,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GraphQlApiEnhancedMetricsConfig {
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
                    r#data_source_level_metrics_behavior: {
                        let field_value = match fields_map.get("data_source_level_metrics_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_source_level_metrics_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operation_level_metrics_config: {
                        let field_value = match fields_map.get("operation_level_metrics_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'operation_level_metrics_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resolver_level_metrics_behavior: {
                        let field_value = match fields_map.get("resolver_level_metrics_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'resolver_level_metrics_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
