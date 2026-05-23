#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualNetworkGatewayVpnClientConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "aadAudience",
                    &self.r#aad_audience,
                ),
                to_pulumi_object_field(
                    "aadIssuer",
                    &self.r#aad_issuer,
                ),
                to_pulumi_object_field(
                    "aadTenant",
                    &self.r#aad_tenant,
                ),
                to_pulumi_object_field(
                    "addressSpaces",
                    &self.r#address_spaces,
                ),
                to_pulumi_object_field(
                    "ipsecPolicy",
                    &self.r#ipsec_policy,
                ),
                to_pulumi_object_field(
                    "radiusServerAddress",
                    &self.r#radius_server_address,
                ),
                to_pulumi_object_field(
                    "radiusServerSecret",
                    &self.r#radius_server_secret,
                ),
                to_pulumi_object_field(
                    "radiusServers",
                    &self.r#radius_servers,
                ),
                to_pulumi_object_field(
                    "revokedCertificates",
                    &self.r#revoked_certificates,
                ),
                to_pulumi_object_field(
                    "rootCertificates",
                    &self.r#root_certificates,
                ),
                to_pulumi_object_field(
                    "virtualNetworkGatewayClientConnections",
                    &self.r#virtual_network_gateway_client_connections,
                ),
                to_pulumi_object_field(
                    "vpnAuthTypes",
                    &self.r#vpn_auth_types,
                ),
                to_pulumi_object_field(
                    "vpnClientProtocols",
                    &self.r#vpn_client_protocols,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualNetworkGatewayVpnClientConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#aad_audience: {
                        let field_value = match fields_map.get("aadAudience") {
                            Some(value) => value,
                            None => bail!("Missing field 'aadAudience' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aad_issuer: {
                        let field_value = match fields_map.get("aadIssuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'aadIssuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aad_tenant: {
                        let field_value = match fields_map.get("aadTenant") {
                            Some(value) => value,
                            None => bail!("Missing field 'aadTenant' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_spaces: {
                        let field_value = match fields_map.get("addressSpaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'addressSpaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_policy: {
                        let field_value = match fields_map.get("ipsecPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsecPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_server_address: {
                        let field_value = match fields_map.get("radiusServerAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'radiusServerAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_server_secret: {
                        let field_value = match fields_map.get("radiusServerSecret") {
                            Some(value) => value,
                            None => bail!("Missing field 'radiusServerSecret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_servers: {
                        let field_value = match fields_map.get("radiusServers") {
                            Some(value) => value,
                            None => bail!("Missing field 'radiusServers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revoked_certificates: {
                        let field_value = match fields_map.get("revokedCertificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'revokedCertificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_certificates: {
                        let field_value = match fields_map.get("rootCertificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'rootCertificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_gateway_client_connections: {
                        let field_value = match fields_map.get("virtualNetworkGatewayClientConnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualNetworkGatewayClientConnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_auth_types: {
                        let field_value = match fields_map.get("vpnAuthTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpnAuthTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_client_protocols: {
                        let field_value = match fields_map.get("vpnClientProtocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpnClientProtocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
