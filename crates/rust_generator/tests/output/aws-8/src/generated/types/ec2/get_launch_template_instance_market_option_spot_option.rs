#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateInstanceMarketOptionSpotOption {
    #[builder(into)]
    #[serde(rename = "blockDurationMinutes")]
    pub r#block_duration_minutes: i32,
    #[builder(into)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: String,
    #[builder(into)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: String,
    #[builder(into)]
    #[serde(rename = "spotInstanceType")]
    pub r#spot_instance_type: String,
    #[builder(into)]
    #[serde(rename = "validUntil")]
    pub r#valid_until: String,
}
