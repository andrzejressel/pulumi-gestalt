#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InsightsReportConfigFrequencyOptionsEndDate {
    /// The day of the month to stop generating inventory reports.
    #[builder(into)]
    #[serde(rename = "day")]
    pub r#day: i32,
    /// The month to stop generating inventory reports.
    #[builder(into)]
    #[serde(rename = "month")]
    pub r#month: i32,
    /// The year to stop generating inventory reports
    #[builder(into)]
    #[serde(rename = "year")]
    pub r#year: i32,
}
