#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnGatewayConnectionVpnLink {
    /// The expected connection bandwidth in MBPS. Defaults to `10`.
    #[builder(into)]
    pub r#bandwidth_mbps: Option<i32>,
    /// Should the BGP be enabled? Defaults to `false`. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into)]
    pub r#bgp_enabled: Option<bool>,
    /// The connection mode of this VPN Link. Possible values are `Default`, `InitiatorOnly` and `ResponderOnly`. Defaults to `Default`.
    #[builder(into)]
    pub r#connection_mode: Option<String>,
    /// One or more `custom_bgp_address` blocks as defined below.
    #[builder(into)]
    pub r#custom_bgp_addresses: Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkCustomBgpAddress>>,
    /// A list of the egress NAT Rule Ids.
    #[builder(into)]
    pub r#egress_nat_rule_ids: Option<Vec<String>>,
    /// A list of the ingress NAT Rule Ids.
    #[builder(into)]
    pub r#ingress_nat_rule_ids: Option<Vec<String>>,
    /// One or more `ipsec_policy` blocks as defined above.
    #[builder(into)]
    pub r#ipsec_policies: Option<Vec<super::super::types::network::VpnGatewayConnectionVpnLinkIpsecPolicy>>,
    /// Whether to use local Azure IP to initiate connection? Defaults to `false`.
    #[builder(into)]
    pub r#local_azure_ip_address_enabled: Option<bool>,
    /// The name which should be used for this VPN Link Connection.
    #[builder(into)]
    pub r#name: String,
    /// Whether to enable policy-based traffic selectors? Defaults to `false`.
    #[builder(into)]
    pub r#policy_based_traffic_selector_enabled: Option<bool>,
    /// The protocol used for this VPN Link Connection. Possible values are `IKEv1` and `IKEv2`. Defaults to `IKEv2`.
    #[builder(into)]
    pub r#protocol: Option<String>,
    /// Should the rate limit be enabled? Defaults to `false`.
    #[builder(into)]
    pub r#ratelimit_enabled: Option<bool>,
    /// Routing weight for this VPN Link Connection. Defaults to `0`.
    #[builder(into)]
    pub r#route_weight: Option<i32>,
    /// SharedKey for this VPN Link Connection.
    #[builder(into)]
    pub r#shared_key: Option<String>,
    /// The ID of the connected VPN Site Link. Changing this forces a new VPN Gateway Connection to be created.
    #[builder(into)]
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
                    "bandwidthMbps",
                    &self.r#bandwidth_mbps,
                ),
                to_pulumi_object_field(
                    "bgpEnabled",
                    &self.r#bgp_enabled,
                ),
                to_pulumi_object_field(
                    "connectionMode",
                    &self.r#connection_mode,
                ),
                to_pulumi_object_field(
                    "customBgpAddresses",
                    &self.r#custom_bgp_addresses,
                ),
                to_pulumi_object_field(
                    "egressNatRuleIds",
                    &self.r#egress_nat_rule_ids,
                ),
                to_pulumi_object_field(
                    "ingressNatRuleIds",
                    &self.r#ingress_nat_rule_ids,
                ),
                to_pulumi_object_field(
                    "ipsecPolicies",
                    &self.r#ipsec_policies,
                ),
                to_pulumi_object_field(
                    "localAzureIpAddressEnabled",
                    &self.r#local_azure_ip_address_enabled,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "policyBasedTrafficSelectorEnabled",
                    &self.r#policy_based_traffic_selector_enabled,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
                to_pulumi_object_field(
                    "ratelimitEnabled",
                    &self.r#ratelimit_enabled,
                ),
                to_pulumi_object_field(
                    "routeWeight",
                    &self.r#route_weight,
                ),
                to_pulumi_object_field(
                    "sharedKey",
                    &self.r#shared_key,
                ),
                to_pulumi_object_field(
                    "vpnSiteLinkId",
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
                        let field_value = match fields_map.get("bandwidthMbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidthMbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bgp_enabled: {
                        let field_value = match fields_map.get("bgpEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgpEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_mode: {
                        let field_value = match fields_map.get("connectionMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_bgp_addresses: {
                        let field_value = match fields_map.get("customBgpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'customBgpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_nat_rule_ids: {
                        let field_value = match fields_map.get("egressNatRuleIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressNatRuleIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ingress_nat_rule_ids: {
                        let field_value = match fields_map.get("ingressNatRuleIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingressNatRuleIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipsec_policies: {
                        let field_value = match fields_map.get("ipsecPolicies") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipsecPolicies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_azure_ip_address_enabled: {
                        let field_value = match fields_map.get("localAzureIpAddressEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'localAzureIpAddressEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("policyBasedTrafficSelectorEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'policyBasedTrafficSelectorEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("ratelimitEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'ratelimitEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_weight: {
                        let field_value = match fields_map.get("routeWeight") {
                            Some(value) => value,
                            None => bail!("Missing field 'routeWeight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#shared_key: {
                        let field_value = match fields_map.get("sharedKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'sharedKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpn_site_link_id: {
                        let field_value = match fields_map.get("vpnSiteLinkId") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpnSiteLinkId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
