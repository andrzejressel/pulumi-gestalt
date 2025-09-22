#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnSiteLink {
    /// A `bgp` block as defined above.
    /// 
    /// > **NOTE:** The `link.bgp` has to be set when the `address_cidrs` isn't specified.
    #[builder(into)]
    #[serde(rename = "bgp")]
    pub r#bgp: Box<Option<super::super::types::network::VpnSiteLinkBgp>>,
    /// The FQDN of this VPN Site Link.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Option<String>,
    /// The ID of the VPN Site Link.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The IP address of this VPN Site Link.
    /// 
    /// > **NOTE:** Either `fqdn` or `ip_address` should be specified.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The name which should be used for this VPN Site Link.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the physical link at the VPN Site. Example: `ATT`, `Verizon`.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: Option<String>,
    /// The speed of the VPN device at the branch location in unit of mbps. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "speedInMbps")]
    pub r#speed_in_mbps: Option<i32>,
}
