#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVpnGatewayBgpSetting {
    /// The ASN of the BGP Speaker.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: i32,
    /// The Address which should be used for the BGP Peering.
    #[builder(into)]
    #[serde(rename = "bgpPeeringAddress")]
    pub r#bgp_peering_address: String,
    /// an `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance0BgpPeeringAddresses")]
    pub r#instance_0_bgp_peering_addresses: Vec<super::super::types::network::GetVpnGatewayBgpSettingInstance0BgpPeeringAddress>,
    /// an `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    #[serde(rename = "instance1BgpPeeringAddresses")]
    pub r#instance_1_bgp_peering_addresses: Vec<super::super::types::network::GetVpnGatewayBgpSettingInstance1BgpPeeringAddress>,
    /// The weight added to Routes learned from this BGP Speaker.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVpnGatewayBgpSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "asn",
                    &self.r#asn,
                ),
                to_pulumi_object_field(
                    "bgp_peering_address",
                    &self.r#bgp_peering_address,
                ),
                to_pulumi_object_field(
                    "instance_0_bgp_peering_addresses",
                    &self.r#instance_0_bgp_peering_addresses,
                ),
                to_pulumi_object_field(
                    "instance_1_bgp_peering_addresses",
                    &self.r#instance_1_bgp_peering_addresses,
                ),
                to_pulumi_object_field(
                    "peer_weight",
                    &self.r#peer_weight,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVpnGatewayBgpSetting {
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
                    r#asn: {
                        let field_value = match fields_map.get("asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bgp_peering_address: {
                        let field_value = match fields_map.get("bgp_peering_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgp_peering_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_0_bgp_peering_addresses: {
                        let field_value = match fields_map.get("instance_0_bgp_peering_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_0_bgp_peering_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_1_bgp_peering_addresses: {
                        let field_value = match fields_map.get("instance_1_bgp_peering_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_1_bgp_peering_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_weight: {
                        let field_value = match fields_map.get("peer_weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
