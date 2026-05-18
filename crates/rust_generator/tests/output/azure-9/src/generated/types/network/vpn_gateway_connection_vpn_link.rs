#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnGatewayConnectionVpnLink {
    /// The expected connection bandwidth in MBPS. Defaults to `10`.
    #[builder(into)]
    #[serde(rename = "bandwidthMbps")]
    pub r#bandwidth_mbps: Option<i32>,
    /// Should the BGP be enabled? Defaults to `false`. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into)]
    #[serde(rename = "bgpEnabled")]
    pub r#bgp_enabled: Option<bool>,
    /// The connection mode of this VPN Link. Possible values are `Default`, `InitiatorOnly` and `ResponderOnly`. Defaults to `Default`.
    #[builder(into)]
    #[serde(rename = "connectionMode")]
    pub r#connection_mode: Option<String>,
    /// One or more `custom_bgp_address` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "customBgpAddresses")]
    pub r#custom_bgp_addresses: Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkCustomBgpAddress>>,
    /// A list of the egress NAT Rule Ids.
    #[builder(into)]
    #[serde(rename = "egressNatRuleIds")]
    pub r#egress_nat_rule_ids: Option<Vec<String>>,
    /// A list of the ingress NAT Rule Ids.
    #[builder(into)]
    #[serde(rename = "ingressNatRuleIds")]
    pub r#ingress_nat_rule_ids: Option<Vec<String>>,
    /// One or more `ipsec_policy` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipsecPolicies")]
    pub r#ipsec_policies: Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkIpsecPolicy>>,
    /// Whether to use local Azure IP to initiate connection? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "localAzureIpAddressEnabled")]
    pub r#local_azure_ip_address_enabled: Option<bool>,
    /// The name which should be used for this VPN Link Connection.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Whether to enable policy-based traffic selectors? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "policyBasedTrafficSelectorEnabled")]
    pub r#policy_based_traffic_selector_enabled: Option<bool>,
    /// The protocol used for this VPN Link Connection. Possible values are `IKEv1` and `IKEv2`. Defaults to `IKEv2`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Should the rate limit be enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ratelimitEnabled")]
    pub r#ratelimit_enabled: Option<bool>,
    /// Routing weight for this VPN Link Connection. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "routeWeight")]
    pub r#route_weight: Option<i32>,
    /// SharedKey for this VPN Link Connection.
    #[builder(into)]
    #[serde(rename = "sharedKey")]
    pub r#shared_key: Option<String>,
    /// The ID of the connected VPN Site Link. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into)]
    #[serde(rename = "vpnSiteLinkId")]
    pub r#vpn_site_link_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnGatewayConnectionVpnLink {
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
                    "bandwidth_mbps",
                    &self.r#bandwidth_mbps,
                ),
                to_pulumi_object_field(
                    "bgp_enabled",
                    &self.r#bgp_enabled,
                ),
                to_pulumi_object_field(
                    "connection_mode",
                    &self.r#connection_mode,
                ),
                to_pulumi_object_field(
                    "custom_bgp_addresses",
                    &self.r#custom_bgp_addresses,
                ),
                to_pulumi_object_field(
                    "egress_nat_rule_ids",
                    &self.r#egress_nat_rule_ids,
                ),
                to_pulumi_object_field(
                    "ingress_nat_rule_ids",
                    &self.r#ingress_nat_rule_ids,
                ),
                to_pulumi_object_field(
                    "ipsec_policies",
                    &self.r#ipsec_policies,
                ),
                to_pulumi_object_field(
                    "local_azure_ip_address_enabled",
                    &self.r#local_azure_ip_address_enabled,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "policy_based_traffic_selector_enabled",
                    &self.r#policy_based_traffic_selector_enabled,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "ratelimit_enabled",
                    &self.r#ratelimit_enabled,
                ),
                to_pulumi_object_field(
                    "route_weight",
                    &self.r#route_weight,
                ),
                to_pulumi_object_field(
                    "shared_key",
                    &self.r#shared_key,
                ),
                to_pulumi_object_field(
                    "vpn_site_link_id",
                    &self.r#vpn_site_link_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnGatewayConnectionVpnLink {
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
                    r#bandwidth_mbps: {
                        let field_value = match fields_map.get("bandwidth_mbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidth_mbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bgp_enabled: {
                        let field_value = match fields_map.get("bgp_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgp_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_mode: {
                        let field_value = match fields_map.get("connection_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_bgp_addresses: {
                        let field_value = match fields_map.get("custom_bgp_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_bgp_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_nat_rule_ids: {
                        let field_value = match fields_map.get("egress_nat_rule_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_nat_rule_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_nat_rule_ids: {
                        let field_value = match fields_map.get("ingress_nat_rule_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_nat_rule_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_policies: {
                        let field_value = match fields_map.get("ipsec_policies") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsec_policies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_azure_ip_address_enabled: {
                        let field_value = match fields_map.get("local_azure_ip_address_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_azure_ip_address_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_based_traffic_selector_enabled: {
                        let field_value = match fields_map.get("policy_based_traffic_selector_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_based_traffic_selector_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ratelimit_enabled: {
                        let field_value = match fields_map.get("ratelimit_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ratelimit_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_weight: {
                        let field_value = match fields_map.get("route_weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_key: {
                        let field_value = match fields_map.get("shared_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'shared_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_site_link_id: {
                        let field_value = match fields_map.get("vpn_site_link_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpn_site_link_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
