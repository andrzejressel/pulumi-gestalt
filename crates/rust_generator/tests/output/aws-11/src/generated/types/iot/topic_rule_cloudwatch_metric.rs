#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleCloudwatchMetric {
    /// The CloudWatch metric name.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The CloudWatch metric namespace name.
    #[builder(into)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: String,
    /// An optional Unix timestamp (http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#about_timestamp).
    #[builder(into)]
    #[serde(rename = "metricTimestamp")]
    pub r#metric_timestamp: Option<String>,
    /// The metric unit (supported units can be found here: http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/cloudwatch_concepts.html#Unit)
    #[builder(into)]
    #[serde(rename = "metricUnit")]
    pub r#metric_unit: String,
    /// The CloudWatch metric value.
    #[builder(into)]
    #[serde(rename = "metricValue")]
    pub r#metric_value: String,
    /// The IAM role ARN that allows access to the CloudWatch metric.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleCloudwatchMetric {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "metric_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_name,
                )
                .await,
            );
            map.insert(
                "metric_namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_namespace,
                )
                .await,
            );
            map.insert(
                "metric_timestamp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_timestamp,
                )
                .await,
            );
            map.insert(
                "metric_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_unit,
                )
                .await,
            );
            map.insert(
                "metric_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_value,
                )
                .await,
            );
            map.insert(
                "role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#role_arn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleCloudwatchMetric {
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
                    r#metric_timestamp: {
                        let field_value = match fields_map.get("metric_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_unit: {
                        let field_value = match fields_map.get("metric_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_value: {
                        let field_value = match fields_map.get("metric_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
