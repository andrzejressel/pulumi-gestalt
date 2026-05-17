#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfiguration {
    #[builder(into)]
    #[serde(rename = "bgpConfigurations")]
    pub r#bgp_configurations: Option<Vec<super::super::types::networkmanager::ConnectPeerConfigurationBgpConfiguration>>,
    /// A Connect peer core network address.
    #[builder(into)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Option<String>,
    /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
    #[builder(into)]
    #[serde(rename = "insideCidrBlocks")]
    pub r#inside_cidr_blocks: Option<Vec<String>>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectPeerConfiguration {
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
                "bgp_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bgp_configurations,
                )
                .await,
            );
            map.insert(
                "core_network_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#core_network_address,
                )
                .await,
            );
            map.insert(
                "inside_cidr_blocks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#inside_cidr_blocks,
                )
                .await,
            );
            map.insert(
                "peer_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#peer_address,
                )
                .await,
            );
            map.insert(
                "protocol".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protocol,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectPeerConfiguration {
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
                    r#bgp_configurations: {
                        let field_value = match fields_map.get("bgp_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgp_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_network_address: {
                        let field_value = match fields_map.get("core_network_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_network_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inside_cidr_blocks: {
                        let field_value = match fields_map.get("inside_cidr_blocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'inside_cidr_blocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_address: {
                        let field_value = match fields_map.get("peer_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
