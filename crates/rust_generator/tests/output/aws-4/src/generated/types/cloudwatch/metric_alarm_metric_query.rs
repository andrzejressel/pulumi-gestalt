#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricAlarmMetricQuery {
    /// The ID of the account where the metrics are located, if this is a cross-account alarm.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// The math expression to be performed on the returned data, if this object is performing a math expression. This expression can use the id of the other metrics to refer to those metrics, and can also use the id of other expressions to use the result of those expressions. For more information about metric math expressions, see Metric Math Syntax and Functions in the [Amazon CloudWatch User Guide](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax).
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// A short name used to tie this object to the results in the response. If you are performing math expressions on this set of data, this name represents that data and can serve as a variable in the mathematical expression. The valid characters are letters, numbers, and underscore. The first character must be a lowercase letter.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know what the value represents.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// The metric to be returned, along with statistics, period, and units. Use this parameter only if this object is retrieving a metric and not performing a math expression on returned data.
    #[builder(into)]
    #[serde(rename = "metric")]
    pub r#metric: Option<Box<super::super::types::cloudwatch::MetricAlarmMetricQueryMetric>>,
    /// Granularity in seconds of returned data points.
    /// For metrics with regular resolution, valid values are any multiple of `60`.
    /// For high-resolution metrics, valid values are `1`, `5`, `10`, `30`, or any multiple of `60`.
    #[builder(into)]
    #[serde(rename = "period")]
    pub r#period: Option<i32>,
    /// Specify exactly one `metric_query` to be `true` to use that `metric_query` result as the alarm.
    /// 
    /// > **NOTE:**  You must specify either `metric` or `expression`. Not both.
    #[builder(into)]
    #[serde(rename = "returnData")]
    pub r#return_data: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricAlarmMetricQuery {
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
                "account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_id,
                )
                .await,
            );
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
                "metric".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric,
                )
                .await,
            );
            map.insert(
                "period".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#period,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricAlarmMetricQuery {
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
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
                    r#metric: {
                        let field_value = match fields_map.get("metric") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#period: {
                        let field_value = match fields_map.get("period") {
                            Some(value) => value,
                            None => bail!("Missing field 'period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
