#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExpressRouteCircuitPeering {
    /// The Either a 16-bit or a 32-bit ASN for Azure.
    #[builder(into)]
    #[serde(rename = "azureAsn")]
    pub r#azure_asn: i32,
    /// The Either a 16-bit or a 32-bit ASN. Can either be public or private.
    #[builder(into)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: i32,
    /// The type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "peeringType")]
    pub r#peering_type: String,
    /// A `/30` subnet for the primary link.
    #[builder(into)]
    #[serde(rename = "primaryPeerAddressPrefix")]
    pub r#primary_peer_address_prefix: String,
    /// A `/30` subnet for the secondary link.
    #[builder(into)]
    #[serde(rename = "secondaryPeerAddressPrefix")]
    pub r#secondary_peer_address_prefix: String,
    /// The shared key. Can be a maximum of 25 characters.
    #[builder(into)]
    #[serde(rename = "sharedKey")]
    pub r#shared_key: String,
    /// A valid VLAN ID to establish this peering on.
    #[builder(into)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetExpressRouteCircuitPeering {
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
                    "azure_asn",
                    &self.r#azure_asn,
                ),
                to_pulumi_object_field(
                    "peer_asn",
                    &self.r#peer_asn,
                ),
                to_pulumi_object_field(
                    "peering_type",
                    &self.r#peering_type,
                ),
                to_pulumi_object_field(
                    "primary_peer_address_prefix",
                    &self.r#primary_peer_address_prefix,
                ),
                to_pulumi_object_field(
                    "secondary_peer_address_prefix",
                    &self.r#secondary_peer_address_prefix,
                ),
                to_pulumi_object_field(
                    "shared_key",
                    &self.r#shared_key,
                ),
                to_pulumi_object_field(
                    "vlan_id",
                    &self.r#vlan_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetExpressRouteCircuitPeering {
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
                    r#azure_asn: {
                        let field_value = match fields_map.get("azure_asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peer_asn: {
                        let field_value = match fields_map.get("peer_asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'peer_asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering_type: {
                        let field_value = match fields_map.get("peering_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_peer_address_prefix: {
                        let field_value = match fields_map.get("primary_peer_address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_peer_address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_peer_address_prefix: {
                        let field_value = match fields_map.get("secondary_peer_address_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_peer_address_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#vlan_id: {
                        let field_value = match fields_map.get("vlan_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'vlan_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
