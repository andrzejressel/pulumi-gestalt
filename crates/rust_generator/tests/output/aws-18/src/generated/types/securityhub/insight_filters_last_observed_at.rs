#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InsightFiltersLastObservedAt {
    /// A configuration block of the date range for the date filter. See date_range below for more details.
    #[builder(into)]
    #[serde(rename = "dateRange")]
    pub r#date_range: Option<Box<super::super::types::securityhub::InsightFiltersLastObservedAtDateRange>>,
    /// An end date for the date filter. Required with `start` if `date_range` is not specified.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Option<String>,
    /// A start date for the date filter. Required with `end` if `date_range` is not specified.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Option<String>,
}
