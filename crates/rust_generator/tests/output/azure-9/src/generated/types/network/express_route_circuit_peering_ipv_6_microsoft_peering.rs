#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExpressRouteCircuitPeeringIpv6MicrosoftPeering {
    /// The communities of Bgp Peering specified for microsoft peering.
    #[builder(into)]
    #[serde(rename = "advertisedCommunities")]
    pub r#advertised_communities: Option<Vec<String>>,
    /// A list of Advertised Public Prefixes.
    #[builder(into)]
    #[serde(rename = "advertisedPublicPrefixes")]
    pub r#advertised_public_prefixes: Option<Vec<String>>,
    /// The CustomerASN of the peering. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "customerAsn")]
    pub r#customer_asn: Option<i32>,
    /// The Routing Registry against which the AS number and prefixes are registered. For example: `ARIN`, `RIPE`, `AFRINIC` etc. Defaults to `NONE`.
    #[builder(into)]
    #[serde(rename = "routingRegistryName")]
    pub r#routing_registry_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExpressRouteCircuitPeeringIpv6MicrosoftPeering {
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
                    "advertised_communities",
                    &self.r#advertised_communities,
                ),
                to_pulumi_object_field(
                    "advertised_public_prefixes",
                    &self.r#advertised_public_prefixes,
                ),
                to_pulumi_object_field(
                    "customer_asn",
                    &self.r#customer_asn,
                ),
                to_pulumi_object_field(
                    "routing_registry_name",
                    &self.r#routing_registry_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExpressRouteCircuitPeeringIpv6MicrosoftPeering {
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
                    r#advertised_communities: {
                        let field_value = match fields_map.get("advertised_communities") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertised_communities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advertised_public_prefixes: {
                        let field_value = match fields_map.get("advertised_public_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertised_public_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_asn: {
                        let field_value = match fields_map.get("customer_asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#routing_registry_name: {
                        let field_value = match fields_map.get("routing_registry_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'routing_registry_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
