#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetZeroTrustInfrastructureAccessTargetsTargetIp {
    /// The target's IPv4 address.
    #[builder(into)]
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Option<Box<super::types::GetZeroTrustInfrastructureAccessTargetsTargetIpIpv4>>,
    /// The target's IPv6 address.
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<Box<super::types::GetZeroTrustInfrastructureAccessTargetsTargetIpIpv6>>,
}
