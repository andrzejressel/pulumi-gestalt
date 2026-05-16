#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReceivedLicenseConsumptionConfigurationBorrowConfiguration {
    /// Indicates whether early check-ins are allowed.
    #[builder(into)]
    #[serde(rename = "allowEarlyCheckIn")]
    pub r#allow_early_check_in: bool,
    /// Maximum time for the provisional configuration, in minutes.
    #[builder(into)]
    #[serde(rename = "maxTimeToLiveInMinutes")]
    pub r#max_time_to_live_in_minutes: i32,
}
