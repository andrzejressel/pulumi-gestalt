#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PurchaseMeterDetailsResponse {
    /// Represents billing type.
    /// Expected value is 'Purchase'.
    #[builder(skip)]
    #[serde(rename = "billingType")]
    r#billing_type: super::constants::ConstStringPurchase,
    /// Charging type.
    #[builder(into)]
    #[serde(rename = "chargingType")]
    pub r#charging_type: String,
    /// Billing unit applicable for Pav2 billing
    #[builder(into)]
    #[serde(rename = "multiplier")]
    pub r#multiplier: f64,
    /// Product Id
    #[builder(into)]
    #[serde(rename = "productId")]
    pub r#product_id: String,
    /// Sku Id
    #[builder(into)]
    #[serde(rename = "skuId")]
    pub r#sku_id: String,
    /// Term Id
    #[builder(into)]
    #[serde(rename = "termId")]
    pub r#term_id: String,
}
