#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VnpGatewayNatRuleExternalMapping {
    /// The string CIDR representing the address space for the VPN Gateway Nat Rule external mapping.
    #[builder(into)]
    #[serde(rename = "addressSpace")]
    pub r#address_space: String,
    /// The single port range for the VPN Gateway Nat Rule external mapping.
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: Option<String>,
}
