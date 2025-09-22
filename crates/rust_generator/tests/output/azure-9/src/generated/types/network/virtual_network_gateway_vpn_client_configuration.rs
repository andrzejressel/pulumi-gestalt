#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkGatewayVpnClientConfiguration {
    /// The client id of the Azure VPN application.
    /// See [Create an Active Directory (AD) tenant for P2S OpenVPN protocol connections](https://docs.microsoft.com/en-gb/azure/vpn-gateway/openvpn-azure-ad-tenant-multi-app) for values
    #[builder(into)]
    #[serde(rename = "aadAudience")]
    pub r#aad_audience: Option<String>,
    /// The STS url for your tenant
    #[builder(into)]
    #[serde(rename = "aadIssuer")]
    pub r#aad_issuer: Option<String>,
    /// AzureAD Tenant URL
    #[builder(into)]
    #[serde(rename = "aadTenant")]
    pub r#aad_tenant: Option<String>,
    /// The address space out of which IP addresses for vpn clients will be taken. You can provide more than one address space, e.g. in CIDR notation.
    #[builder(into)]
    #[serde(rename = "addressSpaces")]
    pub r#address_spaces: Vec<String>,
    /// An `ipsec_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "ipsecPolicy")]
    pub r#ipsec_policy: Option<Box<super::super::types::network::VirtualNetworkGatewayVpnClientConfigurationIpsecPolicy>>,
    /// The address of the Radius server.
    #[builder(into)]
    #[serde(rename = "radiusServerAddress")]
    pub r#radius_server_address: Option<String>,
    /// The secret used by the Radius server.
    #[builder(into)]
    #[serde(rename = "radiusServerSecret")]
    pub r#radius_server_secret: Option<String>,
    /// One or more `radius_server` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "radiusServers")]
    pub r#radius_servers: Option<Vec<super::super::types::network::VirtualNetworkGatewayVpnClientConfigurationRadiusServer>>,
    /// One or more `revoked_certificate` blocks which are defined below.
    #[builder(into)]
    #[serde(rename = "revokedCertificates")]
    pub r#revoked_certificates: Option<Vec<super::super::types::network::VirtualNetworkGatewayVpnClientConfigurationRevokedCertificate>>,
    /// One or more `root_certificate` blocks which are defined below. These root certificates are used to sign the client certificate used by the VPN clients to connect to the gateway.
    #[builder(into)]
    #[serde(rename = "rootCertificates")]
    pub r#root_certificates: Option<Vec<super::super::types::network::VirtualNetworkGatewayVpnClientConfigurationRootCertificate>>,
    /// One or more `virtual_network_gateway_client_connection` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "virtualNetworkGatewayClientConnections")]
    pub r#virtual_network_gateway_client_connections: Option<Vec<super::super::types::network::VirtualNetworkGatewayVpnClientConfigurationVirtualNetworkGatewayClientConnection>>,
    /// List of the vpn authentication types for the virtual network gateway.
    /// The supported values are `AAD`, `Radius` and `Certificate`.
    /// 
    /// > **NOTE:** `vpn_auth_types` must be set when using multiple vpn authentication types.
    #[builder(into)]
    #[serde(rename = "vpnAuthTypes")]
    pub r#vpn_auth_types: Option<Vec<String>>,
    /// List of the protocols supported by the vpn client.
    /// The supported values are `SSTP`, `IkeV2` and `OpenVPN`.
    /// Values `SSTP` and `IkeV2` are incompatible with the use of
    /// `aad_tenant`, `aad_audience` and `aad_issuer`.
    #[builder(into)]
    #[serde(rename = "vpnClientProtocols")]
    pub r#vpn_client_protocols: Option<Vec<String>>,
}
