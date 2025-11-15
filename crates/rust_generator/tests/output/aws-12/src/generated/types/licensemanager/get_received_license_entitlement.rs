#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReceivedLicenseEntitlement {
    /// Indicates whether check-ins are allowed.
    #[builder(into)]
    #[serde(rename = "allowCheckIn")]
    pub r#allow_check_in: bool,
    /// Maximum entitlement count. Use if the unit is not None.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: i32,
    /// The key name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Indicates whether overages are allowed.
    #[builder(into)]
    #[serde(rename = "overage")]
    pub r#overage: bool,
    /// Entitlement unit.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
    /// The value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
