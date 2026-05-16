#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectPeerConfigurationBgpConfiguration {
    /// A Connect peer core network address.
    #[builder(into)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "coreNetworkAsn")]
    pub r#core_network_asn: Option<i32>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Option<String>,
    #[builder(into)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectPeerConfigurationBgpConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("core_network_address".to_string(), self.r#core_network_address.to_pulumi_value().await);
            map.insert("core_network_asn".to_string(), self.r#core_network_asn.to_pulumi_value().await);
            map.insert("peer_address".to_string(), self.r#peer_address.to_pulumi_value().await);
            map.insert("peer_asn".to_string(), self.r#peer_asn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectPeerConfigurationBgpConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#core_network_address: {
                        let field_value = match fields_map.get("core_network_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_network_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#core_network_asn: {
                        let field_value = match fields_map.get("core_network_asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_network_asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#peer_address: {
                        let field_value = match fields_map.get("peer_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#peer_asn: {
                        let field_value = match fields_map.get("peer_asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
