#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile {
    #[builder(into)]
    #[serde(rename = "egressNatIpAddressIds")]
    pub r#egress_nat_ip_address_ids: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "egressNatIpAddresses")]
    pub r#egress_nat_ip_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "ipOfTrustForUserDefinedRoutes")]
    pub r#ip_of_trust_for_user_defined_routes: Option<String>,
    #[builder(into)]
    #[serde(rename = "networkVirtualApplianceId")]
    pub r#network_virtual_appliance_id: String,
    #[builder(into)]
    #[serde(rename = "publicIpAddressIds")]
    pub r#public_ip_address_ids: Vec<String>,
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "trustedAddressRanges")]
    pub r#trusted_address_ranges: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "trustedSubnetId")]
    pub r#trusted_subnet_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "untrustedSubnetId")]
    pub r#untrusted_subnet_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "virtualHubId")]
    pub r#virtual_hub_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile {
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
                "egress_nat_ip_address_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#egress_nat_ip_address_ids,
                )
                .await,
            );
            map.insert(
                "egress_nat_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#egress_nat_ip_addresses,
                )
                .await,
            );
            map.insert(
                "ip_of_trust_for_user_defined_routes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_of_trust_for_user_defined_routes,
                )
                .await,
            );
            map.insert(
                "network_virtual_appliance_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_virtual_appliance_id,
                )
                .await,
            );
            map.insert(
                "public_ip_address_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ip_address_ids,
                )
                .await,
            );
            map.insert(
                "public_ip_addresses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_ip_addresses,
                )
                .await,
            );
            map.insert(
                "trusted_address_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trusted_address_ranges,
                )
                .await,
            );
            map.insert(
                "trusted_subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#trusted_subnet_id,
                )
                .await,
            );
            map.insert(
                "untrusted_subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#untrusted_subnet_id,
                )
                .await,
            );
            map.insert(
                "virtual_hub_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#virtual_hub_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualHubLocalRulestackNetworkProfile {
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
                    r#egress_nat_ip_address_ids: {
                        let field_value = match fields_map.get("egress_nat_ip_address_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_nat_ip_address_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_nat_ip_addresses: {
                        let field_value = match fields_map.get("egress_nat_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'egress_nat_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_of_trust_for_user_defined_routes: {
                        let field_value = match fields_map.get("ip_of_trust_for_user_defined_routes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_of_trust_for_user_defined_routes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_virtual_appliance_id: {
                        let field_value = match fields_map.get("network_virtual_appliance_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_virtual_appliance_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_ids: {
                        let field_value = match fields_map.get("public_ip_address_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_address_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("public_ip_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_ip_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_address_ranges: {
                        let field_value = match fields_map.get("trusted_address_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_address_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_subnet_id: {
                        let field_value = match fields_map.get("trusted_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untrusted_subnet_id: {
                        let field_value = match fields_map.get("untrusted_subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'untrusted_subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_hub_id: {
                        let field_value = match fields_map.get("virtual_hub_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtual_hub_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
