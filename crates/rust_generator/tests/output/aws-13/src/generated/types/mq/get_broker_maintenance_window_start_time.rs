#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetBrokerMaintenanceWindowStartTime {
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: String,
    #[builder(into)]
    #[serde(rename = "timeOfDay")]
    pub r#time_of_day: String,
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
}
