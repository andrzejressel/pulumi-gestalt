#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExpressRouteCircuitPeeringIpv6 {
    /// A boolean value indicating whether the IPv6 peering is enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// A `microsoft_peering` block as defined below.
    #[builder(into)]
    #[serde(rename = "microsoftPeering")]
    pub r#microsoft_peering: Option<Box<super::super::types::network::ExpressRouteCircuitPeeringIpv6MicrosoftPeering>>,
    /// A subnet for the primary link.
    #[builder(into)]
    #[serde(rename = "primaryPeerAddressPrefix")]
    pub r#primary_peer_address_prefix: String,
    /// The ID of the Route Filter. Only available when `peering_type` is set to `MicrosoftPeering`.
    /// 
    /// > **NOTE:** `ipv6` can be specified when `peering_type` is `MicrosoftPeering` or `AzurePrivatePeering`
    #[builder(into)]
    #[serde(rename = "routeFilterId")]
    pub r#route_filter_id: Option<String>,
    /// A subnet for the secondary link.
    #[builder(into)]
    #[serde(rename = "secondaryPeerAddressPrefix")]
    pub r#secondary_peer_address_prefix: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExpressRouteCircuitPeeringIpv6 {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "microsoft_peering",
                    &self.r#microsoft_peering,
                ),
                to_pulumi_object_field(
                    "primary_peer_address_prefix",
                    &self.r#primary_peer_address_prefix,
                ),
                to_pulumi_object_field(
                    "route_filter_id",
                    &self.r#route_filter_id,
                ),
                to_pulumi_object_field(
                    "secondary_peer_address_prefix",
                    &self.r#secondary_peer_address_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExpressRouteCircuitPeeringIpv6 {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#microsoft_peering: {
                        let field_value = match fields_map.get("microsoft_peering") {
                            Some(value) => value,
                            None => bail!("Missing field 'microsoft_peering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#route_filter_id: {
                        let field_value = match fields_map.get("route_filter_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_filter_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
