#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualNetworkGatewayBgpSetting {
    /// The Autonomous System Number (ASN) to use as part of the BGP.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: i32,
    /// The weight added to routes which have been learned
    /// through BGP peering.
    #[builder(into)]
    #[serde(rename = "peerWeight")]
    pub r#peer_weight: i32,
    /// The BGP peer IP address of the virtual network
    /// gateway. This address is needed to configure the created gateway as a BGP Peer
    /// on the on-premises VPN devices.
    #[builder(into)]
    #[serde(rename = "peeringAddress")]
    pub r#peering_address: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetVirtualNetworkGatewayBgpSetting {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "asn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#asn,
                )
                .await,
            );
            map.insert(
                "peer_weight".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peer_weight,
                )
                .await,
            );
            map.insert(
                "peering_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peering_address,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetVirtualNetworkGatewayBgpSetting {
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
                    r#peer_weight: {
                        let field_value = match fields_map.get("peer_weight") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_weight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering_address: {
                        let field_value = match fields_map.get("peering_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
