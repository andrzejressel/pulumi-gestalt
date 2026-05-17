#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduledQueryRulesAlertV2Criteria {
    /// A `dimension` block as defined below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<Vec<super::super::types::monitoring::ScheduledQueryRulesAlertV2CriteriaDimension>>,
    /// A `failing_periods` block as defined below.
    #[builder(into)]
    #[serde(rename = "failingPeriods")]
    pub r#failing_periods: Option<Box<super::super::types::monitoring::ScheduledQueryRulesAlertV2CriteriaFailingPeriods>>,
    /// Specifies the column containing the metric measure number.
    /// 
    /// > **Note** `metric_measure_column` is required if `time_aggregation_method` is `Average`, `Maximum`, `Minimum`, or `Total`. And `metric_measure_column` can not be specified if `time_aggregation_method` is `Count`.
    #[builder(into)]
    #[serde(rename = "metricMeasureColumn")]
    pub r#metric_measure_column: Option<String>,
    /// Specifies the criteria operator. Possible values are `Equal`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan`,and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: String,
    /// The query to run on logs. The results returned by this query are used to populate the alert.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: String,
    /// Specifies the column containing the resource ID. The content of the column must be an uri formatted as resource ID.
    #[builder(into)]
    #[serde(rename = "resourceIdColumn")]
    pub r#resource_id_column: Option<String>,
    /// Specifies the criteria threshold value that activates the alert.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: f64,
    /// The type of aggregation to apply to the data points in aggregation granularity. Possible values are `Average`, `Count`, `Maximum`, `Minimum`,and `Total`.
    #[builder(into)]
    #[serde(rename = "timeAggregationMethod")]
    pub r#time_aggregation_method: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduledQueryRulesAlertV2Criteria {
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
                "dimensions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dimensions,
                )
                .await,
            );
            map.insert(
                "failing_periods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#failing_periods,
                )
                .await,
            );
            map.insert(
                "metric_measure_column".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_measure_column,
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
                "query".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#query,
                )
                .await,
            );
            map.insert(
                "resource_id_column".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_id_column,
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
                "time_aggregation_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_aggregation_method,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduledQueryRulesAlertV2Criteria {
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
                    r#failing_periods: {
                        let field_value = match fields_map.get("failing_periods") {
                            Some(value) => value,
                            None => bail!("Missing field 'failing_periods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_measure_column: {
                        let field_value = match fields_map.get("metric_measure_column") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_measure_column' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#query: {
                        let field_value = match fields_map.get("query") {
                            Some(value) => value,
                            None => bail!("Missing field 'query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_id_column: {
                        let field_value = match fields_map.get("resource_id_column") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_id_column' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#time_aggregation_method: {
                        let field_value = match fields_map.get("time_aggregation_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_aggregation_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
