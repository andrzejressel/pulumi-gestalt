#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualHubPanoramaNetworkProfile {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NextGenerationFirewallVirtualHubPanoramaNetworkProfile {
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
                    "egressNatIpAddressIds",
                    &self.r#egress_nat_ip_address_ids,
                ),
                to_pulumi_object_field(
                    "egressNatIpAddresses",
                    &self.r#egress_nat_ip_addresses,
                ),
                to_pulumi_object_field(
                    "ipOfTrustForUserDefinedRoutes",
                    &self.r#ip_of_trust_for_user_defined_routes,
                ),
                to_pulumi_object_field(
                    "networkVirtualApplianceId",
                    &self.r#network_virtual_appliance_id,
                ),
                to_pulumi_object_field(
                    "publicIpAddressIds",
                    &self.r#public_ip_address_ids,
                ),
                to_pulumi_object_field(
                    "publicIpAddresses",
                    &self.r#public_ip_addresses,
                ),
                to_pulumi_object_field(
                    "trustedAddressRanges",
                    &self.r#trusted_address_ranges,
                ),
                to_pulumi_object_field(
                    "trustedSubnetId",
                    &self.r#trusted_subnet_id,
                ),
                to_pulumi_object_field(
                    "untrustedSubnetId",
                    &self.r#untrusted_subnet_id,
                ),
                to_pulumi_object_field(
                    "virtualHubId",
                    &self.r#virtual_hub_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NextGenerationFirewallVirtualHubPanoramaNetworkProfile {
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
                        let field_value = match fields_map.get("egressNatIpAddressIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressNatIpAddressIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#egress_nat_ip_addresses: {
                        let field_value = match fields_map.get("egressNatIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'egressNatIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_of_trust_for_user_defined_routes: {
                        let field_value = match fields_map.get("ipOfTrustForUserDefinedRoutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipOfTrustForUserDefinedRoutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_virtual_appliance_id: {
                        let field_value = match fields_map.get("networkVirtualApplianceId") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkVirtualApplianceId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_address_ids: {
                        let field_value = match fields_map.get("publicIpAddressIds") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddressIds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_ip_addresses: {
                        let field_value = match fields_map.get("publicIpAddresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'publicIpAddresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_address_ranges: {
                        let field_value = match fields_map.get("trustedAddressRanges") {
                            Some(value) => value,
                            None => bail!("Missing field 'trustedAddressRanges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_subnet_id: {
                        let field_value = match fields_map.get("trustedSubnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'trustedSubnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#untrusted_subnet_id: {
                        let field_value = match fields_map.get("untrustedSubnetId") {
                            Some(value) => value,
                            None => bail!("Missing field 'untrustedSubnetId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#virtual_hub_id: {
                        let field_value = match fields_map.get("virtualHubId") {
                            Some(value) => value,
                            None => bail!("Missing field 'virtualHubId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
