#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetExternalAccessRuleSourceIpRange {
    /// A single IP address.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// An IP address range in the CIDR format.
    #[builder(into)]
    #[serde(rename = "ipAddressRange")]
    pub r#ip_address_range: String,
}
