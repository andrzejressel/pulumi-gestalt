#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SloWindowsBasedSliMetricSumInRangeRange {
    /// max value for the range (inclusive). If not given,
    /// will be set to "infinity", defining an open range
    /// ">= range.min"
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<f64>,
    /// Min value for the range (inclusive). If not given,
    /// will be set to "-infinity", defining an open range
    /// "< range.max"
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Option<f64>,
}
