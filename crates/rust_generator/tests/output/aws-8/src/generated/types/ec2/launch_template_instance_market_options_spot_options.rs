#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LaunchTemplateInstanceMarketOptionsSpotOptions {
    /// The required duration in minutes. This value must be a multiple of 60.
    #[builder(into)]
    #[serde(rename = "blockDurationMinutes")]
    pub r#block_duration_minutes: Option<i32>,
    /// The behavior when a Spot Instance is interrupted. Can be `hibernate`,
    /// `stop`, or `terminate`. (Default: `terminate`).
    #[builder(into)]
    #[serde(rename = "instanceInterruptionBehavior")]
    pub r#instance_interruption_behavior: Option<String>,
    /// The maximum hourly price you're willing to pay for the Spot Instances.
    #[builder(into)]
    #[serde(rename = "maxPrice")]
    pub r#max_price: Option<String>,
    /// The Spot Instance request type. Can be `one-time`, or `persistent`.
    #[builder(into)]
    #[serde(rename = "spotInstanceType")]
    pub r#spot_instance_type: Option<String>,
    /// The end date of the request.
    #[builder(into)]
    #[serde(rename = "validUntil")]
    pub r#valid_until: Option<String>,
}
