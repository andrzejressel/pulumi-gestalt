#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecServiceDiscoveryDns {
    /// DNS host name for your virtual node.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: String,
    /// The preferred IP version that this virtual node uses. Valid values: `IPv6_PREFERRED`, `IPv4_PREFERRED`, `IPv4_ONLY`, `IPv6_ONLY`.
    #[builder(into)]
    #[serde(rename = "ipPreference")]
    pub r#ip_preference: Option<String>,
    /// The DNS response type for the virtual node. Valid values: `LOADBALANCER`, `ENDPOINTS`.
    #[builder(into)]
    #[serde(rename = "responseType")]
    pub r#response_type: Option<String>,
}
