#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Pav2MeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Pav2'.
    #[builder(skip)]
    #[serde(rename = "billingType")]
    r#billing_type: super::constants::ConstStringPav2,
    /// Charging type.
    #[builder(into)]
    #[serde(rename = "chargingType")]
    pub r#charging_type: String,
    /// Validation status of requested data center and transport.
    #[builder(into)]
    #[serde(rename = "meterGuid")]
    pub r#meter_guid: String,
    /// Billing unit applicable for Pav2 billing
    #[builder(into)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: f64,
}
