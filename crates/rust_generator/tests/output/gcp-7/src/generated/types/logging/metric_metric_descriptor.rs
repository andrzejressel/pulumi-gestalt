#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricMetricDescriptor {
    /// A concise name for the metric, which can be displayed in user interfaces. Use sentence case
    /// without an ending period, for example "Request count". This field is optional but it is
    /// recommended to be set for any metrics associated with user-visible concepts, such as Quota.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The set of labels that can be used to describe a specific instance of this metric type. For
    /// example, the appengine.googleapis.com/http/server/response_latencies metric type has a label
    /// for the HTTP response code, response_code, so you can look at latencies for successful responses
    /// or just for responses that failed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<super::super::types::logging::MetricMetricDescriptorLabel>>,
    /// Whether the metric records instantaneous values, changes to a value, etc.
    /// Some combinations of metricKind and valueType might not be supported.
    /// For counter metrics, set this to DELTA.
    /// Possible values are: `DELTA`, `GAUGE`, `CUMULATIVE`.
    #[builder(into)]
    #[serde(rename = "metricKind")]
    pub r#metric_kind: String,
    /// The unit in which the metric value is reported. It is only applicable if the valueType is
    /// `INT64`, `DOUBLE`, or `DISTRIBUTION`. The supported units are a subset of
    /// [The Unified Code for Units of Measure](http://unitsofmeasure.org/ucum.html) standard
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Option<String>,
    /// Whether the measurement is an integer, a floating-point number, etc.
    /// Some combinations of metricKind and valueType might not be supported.
    /// For counter metrics, set this to INT64.
    /// Possible values are: `BOOL`, `INT64`, `DOUBLE`, `STRING`, `DISTRIBUTION`, `MONEY`.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MetricMetricDescriptor {
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
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "metric_kind".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metric_kind,
                )
                .await,
            );
            map.insert(
                "unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unit,
                )
                .await,
            );
            map.insert(
                "value_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MetricMetricDescriptor {
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
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_kind: {
                        let field_value = match fields_map.get("metric_kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'metric_kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#value_type: {
                        let field_value = match fields_map.get("value_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
