#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPolicyConditionConditionThresholdDenominatorAggregation {
    /// The alignment period for per-time
    /// series alignment. If present,
    /// alignmentPeriod must be at least
    /// 60 seconds. After per-time series
    /// alignment, each time series will
    /// contain data points only on the
    /// period boundaries. If
    /// perSeriesAligner is not specified
    /// or equals ALIGN_NONE, then this
    /// field is ignored. If
    /// perSeriesAligner is specified and
    /// does not equal ALIGN_NONE, then
    /// this field must be defined;
    /// otherwise an error is returned.
    #[builder(into)]
    #[serde(rename = "alignmentPeriod")]
    pub r#alignment_period: Option<String>,
    /// The approach to be used to combine
    /// time series. Not all reducer
    /// functions may be applied to all
    /// time series, depending on the
    /// metric type and the value type of
    /// the original time series.
    /// Reduction may change the metric
    /// type of value type of the time
    /// series.Time series data must be
    /// aligned in order to perform cross-
    /// time series reduction. If
    /// crossSeriesReducer is specified,
    /// then perSeriesAligner must be
    /// specified and not equal ALIGN_NONE
    /// and alignmentPeriod must be
    /// specified; otherwise, an error is
    /// returned.
    /// Possible values are: `REDUCE_NONE`, `REDUCE_MEAN`, `REDUCE_MIN`, `REDUCE_MAX`, `REDUCE_SUM`, `REDUCE_STDDEV`, `REDUCE_COUNT`, `REDUCE_COUNT_TRUE`, `REDUCE_COUNT_FALSE`, `REDUCE_FRACTION_TRUE`, `REDUCE_PERCENTILE_99`, `REDUCE_PERCENTILE_95`, `REDUCE_PERCENTILE_50`, `REDUCE_PERCENTILE_05`.
    #[builder(into)]
    #[serde(rename = "crossSeriesReducer")]
    pub r#cross_series_reducer: Option<String>,
    /// The set of fields to preserve when
    /// crossSeriesReducer is specified.
    /// The groupByFields determine how
    /// the time series are partitioned
    /// into subsets prior to applying the
    /// aggregation function. Each subset
    /// contains time series that have the
    /// same value for each of the
    /// grouping fields. Each individual
    /// time series is a member of exactly
    /// one subset. The crossSeriesReducer
    /// is applied to each subset of time
    /// series. It is not possible to
    /// reduce across different resource
    /// types, so this field implicitly
    /// contains resource.type. Fields not
    /// specified in groupByFields are
    /// aggregated away. If groupByFields
    /// is not specified and all the time
    /// series have the same resource
    /// type, then the time series are
    /// aggregated into a single output
    /// time series. If crossSeriesReducer
    /// is not defined, this field is
    /// ignored.
    #[builder(into)]
    #[serde(rename = "groupByFields")]
    pub r#group_by_fields: Option<Vec<String>>,
    /// The approach to be used to align
    /// individual time series. Not all
    /// alignment functions may be applied
    /// to all time series, depending on
    /// the metric type and value type of
    /// the original time series.
    /// Alignment may change the metric
    /// type or the value type of the time
    /// series.Time series data must be
    /// aligned in order to perform cross-
    /// time series reduction. If
    /// crossSeriesReducer is specified,
    /// then perSeriesAligner must be
    /// specified and not equal ALIGN_NONE
    /// and alignmentPeriod must be
    /// specified; otherwise, an error is
    /// returned.
    /// Possible values are: `ALIGN_NONE`, `ALIGN_DELTA`, `ALIGN_RATE`, `ALIGN_INTERPOLATE`, `ALIGN_NEXT_OLDER`, `ALIGN_MIN`, `ALIGN_MAX`, `ALIGN_MEAN`, `ALIGN_COUNT`, `ALIGN_SUM`, `ALIGN_STDDEV`, `ALIGN_COUNT_TRUE`, `ALIGN_COUNT_FALSE`, `ALIGN_FRACTION_TRUE`, `ALIGN_PERCENTILE_99`, `ALIGN_PERCENTILE_95`, `ALIGN_PERCENTILE_50`, `ALIGN_PERCENTILE_05`, `ALIGN_PERCENT_CHANGE`.
    #[builder(into)]
    #[serde(rename = "perSeriesAligner")]
    pub r#per_series_aligner: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertPolicyConditionConditionThresholdDenominatorAggregation {
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
                    "alignment_period",
                    &self.r#alignment_period,
                ),
                to_pulumi_object_field(
                    "cross_series_reducer",
                    &self.r#cross_series_reducer,
                ),
                to_pulumi_object_field(
                    "group_by_fields",
                    &self.r#group_by_fields,
                ),
                to_pulumi_object_field(
                    "per_series_aligner",
                    &self.r#per_series_aligner,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertPolicyConditionConditionThresholdDenominatorAggregation {
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
                    r#alignment_period: {
                        let field_value = match fields_map.get("alignment_period") {
                            Some(value) => value,
                            None => bail!("Missing field 'alignment_period' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cross_series_reducer: {
                        let field_value = match fields_map.get("cross_series_reducer") {
                            Some(value) => value,
                            None => bail!("Missing field 'cross_series_reducer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_by_fields: {
                        let field_value = match fields_map.get("group_by_fields") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_by_fields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#per_series_aligner: {
                        let field_value = match fields_map.get("per_series_aligner") {
                            Some(value) => value,
                            None => bail!("Missing field 'per_series_aligner' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
