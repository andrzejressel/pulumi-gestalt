#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloWindowsBasedSli {
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// with ValueType = BOOL. The window is good if any true values
    /// appear in the window. One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    #[builder(into)]
    pub r#good_bad_metric_filter: Option<String>,
    /// Criterion that describes a window as good if its performance is
    /// high enough. One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Structure is documented below.
    #[builder(into)]
    pub r#good_total_ratio_threshold: Option<Box<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThreshold>>,
    /// Criterion that describes a window as good if the metric's value
    /// is in a good range, *averaged* across returned streams.
    /// One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Average value X of `time_series` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// Structure is documented below.
    #[builder(into)]
    pub r#metric_mean_in_range: Option<Box<super::super::types::monitoring::SloWindowsBasedSliMetricMeanInRange>>,
    /// Criterion that describes a window as good if the metric's value
    /// is in a good range, *summed* across returned streams.
    /// Summed value `X` of `time_series` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Structure is documented below.
    #[builder(into)]
    pub r#metric_sum_in_range: Option<Box<super::super::types::monitoring::SloWindowsBasedSliMetricSumInRange>>,
    /// Duration over which window quality is evaluated, given as a
    /// duration string "{X}s" representing X seconds. Must be an
    /// integer fraction of a day and at least 60s.
    #[builder(into)]
    pub r#window_period: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SloWindowsBasedSli {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "goodBadMetricFilter",
                    &self.r#good_bad_metric_filter,
                ),
                to_pulumi_object_field(
                    "goodTotalRatioThreshold",
                    &self.r#good_total_ratio_threshold,
                ),
                to_pulumi_object_field(
                    "metricMeanInRange",
                    &self.r#metric_mean_in_range,
                ),
                to_pulumi_object_field(
                    "metricSumInRange",
                    &self.r#metric_sum_in_range,
                ),
                to_pulumi_object_field(
                    "windowPeriod",
                    &self.r#window_period,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SloWindowsBasedSli {
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
                    r#good_bad_metric_filter: {
                        let field_value = match fields_map.get("goodBadMetricFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'goodBadMetricFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#good_total_ratio_threshold: {
                        let field_value = match fields_map.get("goodTotalRatioThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'goodTotalRatioThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_mean_in_range: {
                        let field_value = match fields_map.get("metricMeanInRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricMeanInRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metric_sum_in_range: {
                        let field_value = match fields_map.get("metricSumInRange") {
                            Some(value) => value,
                            None => bail!("Missing field 'metricSumInRange' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#window_period: {
                        let field_value = match fields_map.get("windowPeriod") {
                            Some(value) => value,
                            None => bail!("Missing field 'windowPeriod' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
