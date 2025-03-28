#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertPolicyConditionConditionAbsentAggregation {
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
    #[builder(into, default)]
    #[serde(rename = "alignmentPeriod")]
    pub r#alignment_period: Box<Option<String>>,
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
    #[builder(into, default)]
    #[serde(rename = "crossSeriesReducer")]
    pub r#cross_series_reducer: Box<Option<String>>,
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
    #[builder(into, default)]
    #[serde(rename = "groupByFields")]
    pub r#group_by_fields: Box<Option<Vec<String>>>,
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
    #[builder(into, default)]
    #[serde(rename = "perSeriesAligner")]
    pub r#per_series_aligner: Box<Option<String>>,
}
