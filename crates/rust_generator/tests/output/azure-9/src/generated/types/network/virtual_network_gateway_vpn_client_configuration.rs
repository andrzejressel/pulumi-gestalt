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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "aad_audience".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aad_audience,
                )
                .await,
            );
            map.insert(
                "aad_issuer".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aad_issuer,
                )
                .await,
            );
            map.insert(
                "aad_tenant".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aad_tenant,
                )
                .await,
            );
            map.insert(
                "address_spaces".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_spaces,
                )
                .await,
            );
            map.insert(
                "ipsec_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ipsec_policy,
                )
                .await,
            );
            map.insert(
                "radius_server_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_server_address,
                )
                .await,
            );
            map.insert(
                "radius_server_secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_server_secret,
                )
                .await,
            );
            map.insert(
                "radius_servers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#radius_servers,
                )
                .await,
            );
            map.insert(
                "revoked_certificates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#revoked_certificates,
                )
                .await,
            );
            map.insert(
                "root_certificates".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#root_certificates,
                )
                .await,
            );
            map.insert(
                "virtual_network_gateway_client_connections".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_network_gateway_client_connections,
                )
                .await,
            );
            map.insert(
                "vpn_auth_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpn_auth_types,
                )
                .await,
            );
            map.insert(
                "vpn_client_protocols".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpn_client_protocols,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("aad_audience") {
                            Some(value) => value,
                            None => bail!("Missing field 'aad_audience' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aad_issuer: {
                        let field_value = match fields_map.get("aad_issuer") {
                            Some(value) => value,
                            None => bail!("Missing field 'aad_issuer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#aad_tenant: {
                        let field_value = match fields_map.get("aad_tenant") {
                            Some(value) => value,
                            None => bail!("Missing field 'aad_tenant' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_spaces: {
                        let field_value = match fields_map.get("address_spaces") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_spaces' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_policy: {
                        let field_value = match fields_map.get("ipsec_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsec_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_server_address: {
                        let field_value = match fields_map.get("radius_server_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_server_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_server_secret: {
                        let field_value = match fields_map.get("radius_server_secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_server_secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#radius_servers: {
                        let field_value = match fields_map.get("radius_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'radius_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#revoked_certificates: {
                        let field_value = match fields_map.get("revoked_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'revoked_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#root_certificates: {
                        let field_value = match fields_map.get("root_certificates") {
                            Some(value) => value,
                            None => bail!("Missing field 'root_certificates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_network_gateway_client_connections: {
                        let field_value = match fields_map.get("virtual_network_gateway_client_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_network_gateway_client_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_auth_types: {
                        let field_value = match fields_map.get("vpn_auth_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_auth_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_client_protocols: {
                        let field_value = match fields_map.get("vpn_client_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_client_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
