#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricAlertApplicationInsightsWebTestLocationAvailabilityCriteria {
    /// The ID of the Application Insights Resource.
    #[builder(into)]
    #[serde(rename = "componentId")]
    pub r#component_id: String,
    /// The number of failed locations.
    #[builder(into)]
    #[serde(rename = "failedLocationCount")]
    pub r#failed_location_count: i32,
    /// The ID of the Application Insights Web Test.
    #[builder(into)]
    #[serde(rename = "webTestId")]
    pub r#web_test_id: String,
}
