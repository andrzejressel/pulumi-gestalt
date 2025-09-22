#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ZeroTrustGatewayPolicyRuleSettingsDnsResolvers {
    /// IPv4 resolvers.
    #[builder(into)]
    #[serde(rename = "ipv4s")]
    pub r#ipv_4_s: Option<Vec<super::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv4>>,
    /// IPv6 resolvers.
    #[builder(into)]
    #[serde(rename = "ipv6s")]
    pub r#ipv_6_s: Option<Vec<super::types::ZeroTrustGatewayPolicyRuleSettingsDnsResolversIpv6>>,
}
