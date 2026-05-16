#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricAlarmMetricQueryMetric {
    /// The dimensions for this metric.  For the list of available dimensions see the AWS documentation [here](http://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<std::collections::HashMap<String, String>>,
    /// The name for this metric.
    /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: String,
    /// The namespace for this metric. See docs for the [list of namespaces](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/aws-namespaces.html).
    /// See docs for [supported metrics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/CW_Support_For_AWS.html).
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
    /// Granularity in seconds of returned data points.
    /// For metrics with regular resolution, valid values are any multiple of `60`.
    /// For high-resolution metrics, valid values are `1`, `5`, `10`, `30`, or any multiple of `60`.
    #[builder(into)]
    #[serde(rename = "period")]
    pub r#period: i32,
    /// The statistic to apply to this metric.
    /// See docs for [supported statistics](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/Statistics-definitions.html).
    #[builder(into)]
    #[serde(rename = "stat")]
    pub r#stat: String,
    /// The unit for this metric.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricAlarmMetricQueryMetric {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dimensions".to_string(), self.r#dimensions.to_pulumi_value().await);
            map.insert("metric_name".to_string(), self.r#metric_name.to_pulumi_value().await);
            map.insert("namespace".to_string(), self.r#namespace.to_pulumi_value().await);
            map.insert("period".to_string(), self.r#period.to_pulumi_value().await);
            map.insert("stat".to_string(), self.r#stat.to_pulumi_value().await);
            map.insert("unit".to_string(), self.r#unit.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricAlarmMetricQueryMetric {
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
                    r#dimensions: {
                        let field_value = match fields_map.get("dimensions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metric_name: {
                        let field_value = match fields_map.get("metric_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#namespace: {
                        let field_value = match fields_map.get("namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#period: {
                        let field_value = match fields_map.get("period") {
                            Some(value) => value,
                            None => bail!("Missing field 'period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stat: {
                        let field_value = match fields_map.get("stat") {
                            Some(value) => value,
                            None => bail!("Missing field 'stat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#unit: {
                        let field_value = match fields_map.get("unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
