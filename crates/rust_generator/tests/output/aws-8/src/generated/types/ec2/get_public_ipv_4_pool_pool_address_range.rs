#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPublicIpv4PoolPoolAddressRange {
    /// Number of addresses in the range.
    #[builder(into)]
    #[serde(rename = "addressCount")]
    pub r#address_count: i32,
    /// Number of available addresses in the range.
    #[builder(into)]
    #[serde(rename = "availableAddressCount")]
    pub r#available_address_count: i32,
    /// First address in the range.
    #[builder(into)]
    #[serde(rename = "firstAddress")]
    pub r#first_address: String,
    /// Last address in the range.
    #[builder(into)]
    #[serde(rename = "lastAddress")]
    pub r#last_address: String,
}
