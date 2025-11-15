#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SloWindowsBasedSliGoodTotalRatioThreshold {
    /// Basic SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "basicSliPerformance")]
    pub r#basic_sli_performance: Option<Box<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdBasicSliPerformance>>,
    /// Request-based SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "performance")]
    pub r#performance: Option<Box<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdPerformance>>,
    /// If window performance >= threshold, the window is counted
    /// as good.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Option<f64>,
}
