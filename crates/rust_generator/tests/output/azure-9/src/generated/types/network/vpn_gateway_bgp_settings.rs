#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnGatewayBgpSettings {
    /// The ASN of the BGP Speaker. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#asn: i32,
    /// The Address which should be used for the BGP Peering.
    #[builder(into)]
    pub r#bgp_peering_address: Option<String>,
    /// An `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    pub r#instance_0_bgp_peering_address: Option<Box<super::super::types::network::VpnGatewayBgpSettingsInstance0BgpPeeringAddress>>,
    /// An `instance_bgp_peering_address` block as defined below.
    #[builder(into)]
    pub r#instance_1_bgp_peering_address: Option<Box<super::super::types::network::VpnGatewayBgpSettingsInstance1BgpPeeringAddress>>,
    /// The weight added to Routes learned from this BGP Speaker. Changing this forces a new resource to be created.
    #[builder(into)]
    pub r#peer_weight: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnGatewayBgpSettings {
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
                    "asn",
                    &self.r#asn,
                ),
                to_pulumi_object_field(
                    "bgpPeeringAddress",
                    &self.r#bgp_peering_address,
                ),
                to_pulumi_object_field(
                    "instance0BgpPeeringAddress",
                    &self.r#instance_0_bgp_peering_address,
                ),
                to_pulumi_object_field(
                    "instance1BgpPeeringAddress",
                    &self.r#instance_1_bgp_peering_address,
                ),
                to_pulumi_object_field(
                    "peerWeight",
                    &self.r#peer_weight,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnGatewayBgpSettings {
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
                        let field_value = match fields_map.get("bgpPeeringAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgpPeeringAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_0_bgp_peering_address: {
                        let field_value = match fields_map.get("instance0BgpPeeringAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance0BgpPeeringAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_1_bgp_peering_address: {
                        let field_value = match fields_map.get("instance1BgpPeeringAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance1BgpPeeringAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_weight: {
                        let field_value = match fields_map.get("peerWeight") {
                            Some(value) => value,
                            None => bail!("Missing field 'peerWeight' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
