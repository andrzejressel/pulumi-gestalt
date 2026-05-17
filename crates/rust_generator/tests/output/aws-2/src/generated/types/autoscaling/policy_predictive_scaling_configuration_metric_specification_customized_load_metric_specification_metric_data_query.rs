#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQuery {
    /// Math expression used on the returned metric. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// Short name for the metric used in predictive scaling policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Human-readable label for this metric or expression.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Structure that defines CloudWatch metric to be used in predictive scaling policy. You must specify either `expression` or `metric_stat`, but not both.
    #[builder(into)]
    #[serde(rename = "metricStat")]
    pub r#metric_stat: Option<Box<super::super::types::autoscaling::PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQueryMetricStat>>,
    /// Boolean that indicates whether to return the timestamps and raw data values of this metric, the default is true
    #[builder(into)]
    #[serde(rename = "returnData")]
    pub r#return_data: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQuery {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "expression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#expression,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label,
                )
                .await,
            );
            map.insert(
                "metric_stat".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_stat,
                )
                .await,
            );
            map.insert(
                "return_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#return_data,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PolicyPredictiveScalingConfigurationMetricSpecificationCustomizedLoadMetricSpecificationMetricDataQuery {
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
                    r#expression: {
                        let field_value = match fields_map.get("expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_stat: {
                        let field_value = match fields_map.get("metric_stat") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_stat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#return_data: {
                        let field_value = match fields_map.get("return_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'return_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
