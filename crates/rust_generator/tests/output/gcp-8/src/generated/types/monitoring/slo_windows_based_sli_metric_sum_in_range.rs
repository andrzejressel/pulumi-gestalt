#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloWindowsBasedSliMetricSumInRange {
    /// Range of numerical values. The computed good_service
    /// will be the count of values x in the Distribution such
    /// that range.min <= x <= range.max. inclusive of min and
    /// max. Open ranges can be defined by setting
    /// just one of min or max. Summed value `X` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<super::super::types::monitoring::SloWindowsBasedSliMetricSumInRangeRange>,
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying the TimeSeries to use for evaluating window
    /// quality. The provided TimeSeries must have
    /// ValueType = INT64 or ValueType = DOUBLE and
    /// MetricKind = GAUGE.
    /// Summed value `X` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    #[builder(into)]
    #[serde(rename = "timeSeries")]
    pub r#time_series: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloWindowsBasedSliMetricSumInRange {
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
                "range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#range,
                )
                .await,
            );
            map.insert(
                "time_series".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#time_series,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloWindowsBasedSliMetricSumInRange {
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
                    r#range: {
                        let field_value = match fields_map.get("range") {
                            Some(value) => value,
                            None => bail!("Missing field 'range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_series: {
                        let field_value = match fields_map.get("time_series") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_series' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
