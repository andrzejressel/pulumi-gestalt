#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionThreshold {
    /// Specifies the alignment of data points in
    /// individual time series as well as how to
    /// combine the retrieved time series together
    /// (such as when aggregating multiple streams
    /// on each resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).
    /// Multiple aggregations are applied in the
    /// order specified.This field is similar to the
    /// one in the MetricService.ListTimeSeries
    /// request. It is advisable to use the
    /// ListTimeSeries method when debugging this
    /// field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionThresholdAggregation>>,
    /// The comparison to apply between the time
    /// series (indicated by filter and aggregation)
    /// and the threshold (indicated by
    /// threshold_value). The comparison is applied
    /// on each time series, with the time series on
    /// the left-hand side and the threshold on the
    /// right-hand side. Only COMPARISON_LT and
    /// COMPARISON_GT are supported currently.
    /// Possible values are: `COMPARISON_GT`, `COMPARISON_GE`, `COMPARISON_LT`, `COMPARISON_LE`, `COMPARISON_EQ`, `COMPARISON_NE`.
    #[builder(into)]
    #[serde(rename = "comparison")]
    pub r#comparison: String,
    /// Specifies the alignment of data points in
    /// individual time series selected by
    /// denominatorFilter as well as how to combine
    /// the retrieved time series together (such as
    /// when aggregating multiple streams on each
    /// resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).When
    /// computing ratios, the aggregations and
    /// denominator_aggregations fields must use the
    /// same alignment period and produce time
    /// series that have the same periodicity and
    /// labels.This field is similar to the one in
    /// the MetricService.ListTimeSeries request. It
    /// is advisable to use the ListTimeSeries
    /// method when debugging this field.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "denominatorAggregations")]
    pub r#denominator_aggregations: Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionThresholdDenominatorAggregation>>,
    /// A filter that identifies a time series that
    /// should be used as the denominator of a ratio
    /// that will be compared with the threshold. If
    /// a denominator_filter is specified, the time
    /// series specified by the filter field will be
    /// used as the numerator.The filter is similar
    /// to the one that is specified in the
    /// MetricService.ListTimeSeries request (that
    /// call is useful to verify the time series
    /// that will be retrieved / processed) and must
    /// specify the metric type and optionally may
    /// contain restrictions on resource type,
    /// resource labels, and metric labels. This
    /// field may not exceed 2048 Unicode characters
    /// in length.
    #[builder(into)]
    #[serde(rename = "denominatorFilter")]
    pub r#denominator_filter: Option<String>,
    /// The amount of time that a time series must
    /// violate the threshold to be considered
    /// failing. Currently, only values that are a
    /// multiple of a minute--e.g., 0, 60, 120, or
    /// 300 seconds--are supported. If an invalid
    /// value is given, an error will be returned.
    /// When choosing a duration, it is useful to
    /// keep in mind the frequency of the underlying
    /// time series data (which may also be affected
    /// by any alignments specified in the
    /// aggregations field); a good duration is long
    /// enough so that a single outlier does not
    /// generate spurious alerts, but short enough
    /// that unhealthy states are detected and
    /// alerted on quickly.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: String,
    /// A condition control that determines how
    /// metric-threshold conditions are evaluated when
    /// data stops arriving.
    /// Possible values are: `EVALUATION_MISSING_DATA_INACTIVE`, `EVALUATION_MISSING_DATA_ACTIVE`, `EVALUATION_MISSING_DATA_NO_OP`.
    #[builder(into)]
    #[serde(rename = "evaluationMissingData")]
    pub r#evaluation_missing_data: Option<String>,
    /// A filter that identifies which time series
    /// should be compared with the threshold.The
    /// filter is similar to the one that is
    /// specified in the
    /// MetricService.ListTimeSeries request (that
    /// call is useful to verify the time series
    /// that will be retrieved / processed) and must
    /// specify the metric type and optionally may
    /// contain restrictions on resource type,
    /// resource labels, and metric labels. This
    /// field may not exceed 2048 Unicode characters
    /// in length.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// When this field is present, the `MetricThreshold`
    /// condition forecasts whether the time series is
    /// predicted to violate the threshold within the
    /// `forecastHorizon`. When this field is not set, the
    /// `MetricThreshold` tests the current value of the
    /// timeseries against the threshold.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "forecastOptions")]
    pub r#forecast_options: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionThresholdForecastOptions>>,
    /// A value against which to compare the time
    /// series.
    #[builder(into)]
    #[serde(rename = "thresholdValue")]
    pub r#threshold_value: Option<f64>,
    /// The number/percent of time series for which
    /// the comparison must hold in order for the
    /// condition to trigger. If unspecified, then
    /// the condition will trigger if the comparison
    /// is true for any of the time series that have
    /// been identified by filter and aggregations,
    /// or by the ratio, if denominator_filter and
    /// denominator_aggregations are specified.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<Box<super::super::types::monitoring::AlertPolicyConditionConditionThresholdTrigger>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyConditionConditionThreshold {
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
                "aggregations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregations,
                )
                .await,
            );
            map.insert(
                "comparison".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#comparison,
                )
                .await,
            );
            map.insert(
                "denominator_aggregations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#denominator_aggregations,
                )
                .await,
            );
            map.insert(
                "denominator_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#denominator_filter,
                )
                .await,
            );
            map.insert(
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "evaluation_missing_data".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#evaluation_missing_data,
                )
                .await,
            );
            map.insert(
                "filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter,
                )
                .await,
            );
            map.insert(
                "forecast_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forecast_options,
                )
                .await,
            );
            map.insert(
                "threshold_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threshold_value,
                )
                .await,
            );
            map.insert(
                "trigger".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trigger,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyConditionConditionThreshold {
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
                    r#aggregations: {
                        let field_value = match fields_map.get("aggregations") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#comparison: {
                        let field_value = match fields_map.get("comparison") {
                            Some(value) => value,
                            None => bail!("Missing field 'comparison' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denominator_aggregations: {
                        let field_value = match fields_map.get("denominator_aggregations") {
                            Some(value) => value,
                            None => bail!("Missing field 'denominator_aggregations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denominator_filter: {
                        let field_value = match fields_map.get("denominator_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'denominator_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#evaluation_missing_data: {
                        let field_value = match fields_map.get("evaluation_missing_data") {
                            Some(value) => value,
                            None => bail!("Missing field 'evaluation_missing_data' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forecast_options: {
                        let field_value = match fields_map.get("forecast_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'forecast_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threshold_value: {
                        let field_value = match fields_map.get("threshold_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'threshold_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trigger: {
                        let field_value = match fields_map.get("trigger") {
                            Some(value) => value,
                            None => bail!("Missing field 'trigger' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
