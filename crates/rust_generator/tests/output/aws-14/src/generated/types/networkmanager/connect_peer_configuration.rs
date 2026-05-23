#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfiguration {
    #[builder(into)]
    pub r#bgp_configurations: Option<Vec<super::super::types::networkmanager::ConnectPeerConfigurationBgpConfiguration>>,
    /// A Connect peer core network address.
    #[builder(into)]
    pub r#core_network_address: Option<String>,
    /// The inside IP addresses used for BGP peering. Required when the Connect attachment protocol is `GRE`. See `aws.networkmanager.ConnectAttachment` for details.
    #[builder(into)]
    pub r#inside_cidr_blocks: Option<Vec<String>>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    pub r#protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectPeerConfiguration {
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
                    "bgpConfigurations",
                    &self.r#bgp_configurations,
                ),
                to_pulumi_object_field(
                    "coreNetworkAddress",
                    &self.r#core_network_address,
                ),
                to_pulumi_object_field(
                    "insideCidrBlocks",
                    &self.r#inside_cidr_blocks,
                ),
                to_pulumi_object_field(
                    "peerAddress",
                    &self.r#peer_address,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("bgpConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgpConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_network_address: {
                        let field_value = match fields_map.get("coreNetworkAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'coreNetworkAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inside_cidr_blocks: {
                        let field_value = match fields_map.get("insideCidrBlocks") {
                            Some(value) => value,
                            None => bail!("Missing field 'insideCidrBlocks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_address: {
                        let field_value = match fields_map.get("peerAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'peerAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
