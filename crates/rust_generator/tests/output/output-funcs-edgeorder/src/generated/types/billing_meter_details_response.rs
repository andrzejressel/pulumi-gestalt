#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BillingMeterDetailsResponse {
    /// Frequency of recurrence
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// Represents MeterDetails
    #[builder(into)]
    #[serde(rename = "meterDetails")]
    pub r#meter_details: pulumi_gestalt_rust::OneOf2<Box<super::types::Pav2MeterDetailsResponse>, Box<super::types::PurchaseMeterDetailsResponse>>,
    /// Represents Metering type (eg one-time or recurrent)
    #[builder(into)]
    #[serde(rename = "meteringType")]
    pub r#metering_type: String,
    /// Represents Billing type name
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
