#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouterStatusBestRoute {
    /// An optional description of this resource. Provide this property
    /// when you create the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The destination range of outgoing packets that this route applies to.
    /// Only IPv4 is supported.
    #[builder(into)]
    #[serde(rename = "destRange")]
    pub r#dest_range: String,
    /// The name of the router.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The network name or resource link to the parent
    /// network of this subnetwork.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// URL to a gateway that should handle matching packets.
    /// Currently, you can only specify the internet gateway, using a full or
    /// partial valid URL:
    /// * 'https://www.googleapis.com/compute/v1/projects/project/global/gateways/default-internet-gateway'
    /// * 'projects/project/global/gateways/default-internet-gateway'
    /// * 'global/gateways/default-internet-gateway'
    /// * The string 'default-internet-gateway'.
    #[builder(into)]
    #[serde(rename = "nextHopGateway")]
    pub r#next_hop_gateway: String,
    /// The IP address or URL to a forwarding rule of type
    /// loadBalancingScheme=INTERNAL that should handle matching
    /// packets.
    /// 
    /// With the GA provider you can only specify the forwarding
    /// rule as a partial or full URL. For example, the following
    /// are all valid values:
    /// * 10.128.0.56
    /// * https://www.googleapis.com/compute/v1/projects/project/regions/region/forwardingRules/forwardingRule
    /// * regions/region/forwardingRules/forwardingRule
    /// 
    /// When the beta provider, you can also specify the IP address
    /// of a forwarding rule from the same VPC or any peered VPC.
    /// 
    /// Note that this can only be used when the destinationRange is
    /// a public (non-RFC 1918) IP CIDR range.
    #[builder(into)]
    #[serde(rename = "nextHopIlb")]
    pub r#next_hop_ilb: String,
    /// URL to an instance that should handle matching packets.
    /// You can specify this as a full or partial URL. For example:
    /// * 'https://www.googleapis.com/compute/v1/projects/project/zones/zone/instances/instance'
    /// * 'projects/project/zones/zone/instances/instance'
    /// * 'zones/zone/instances/instance'
    /// * Just the instance name, with the zone in 'next_hop_instance_zone'.
    #[builder(into)]
    #[serde(rename = "nextHopInstance")]
    pub r#next_hop_instance: String,
    /// The zone of the instance specified in next_hop_instance. Omit if next_hop_instance is specified as a URL.
    #[builder(into)]
    #[serde(rename = "nextHopInstanceZone")]
    pub r#next_hop_instance_zone: String,
    /// Internal fixed region-to-region cost that Google Cloud calculates based on factors such as network performance, distance, and available bandwidth between regions.
    #[builder(into)]
    #[serde(rename = "nextHopInterRegionCost")]
    pub r#next_hop_inter_region_cost: String,
    /// Network IP address of an instance that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopIp")]
    pub r#next_hop_ip: String,
    /// Multi-Exit Discriminator, a BGP route metric that indicates the desirability of a particular route in a network.
    #[builder(into)]
    #[serde(rename = "nextHopMed")]
    pub r#next_hop_med: String,
    /// URL to a Network that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopNetwork")]
    pub r#next_hop_network: String,
    /// Indicates the origin of the route. Can be IGP (Interior Gateway Protocol), EGP (Exterior Gateway Protocol), or INCOMPLETE.
    #[builder(into)]
    #[serde(rename = "nextHopOrigin")]
    pub r#next_hop_origin: String,
    /// URL to a VpnTunnel that should handle matching packets.
    #[builder(into)]
    #[serde(rename = "nextHopVpnTunnel")]
    pub r#next_hop_vpn_tunnel: String,
    /// The priority of this route. Priority is used to break ties in cases
    /// where there is more than one matching route of equal prefix length.
    /// 
    /// In the case of two routes with equal prefix length, the one with the
    /// lowest-numbered priority value wins.
    /// 
    /// Default value is 1000. Valid range is 0 through 65535.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The ID of the project in which the resource
    /// belongs. If it is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
    /// A list of instance tags to which this route applies.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RouterStatusBestRoute {
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
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "dest_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dest_range,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network,
                )
                .await,
            );
            map.insert(
                "next_hop_gateway".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_gateway,
                )
                .await,
            );
            map.insert(
                "next_hop_ilb".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_ilb,
                )
                .await,
            );
            map.insert(
                "next_hop_instance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_instance,
                )
                .await,
            );
            map.insert(
                "next_hop_instance_zone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_instance_zone,
                )
                .await,
            );
            map.insert(
                "next_hop_inter_region_cost".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_inter_region_cost,
                )
                .await,
            );
            map.insert(
                "next_hop_ip".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_ip,
                )
                .await,
            );
            map.insert(
                "next_hop_med".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_med,
                )
                .await,
            );
            map.insert(
                "next_hop_network".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_network,
                )
                .await,
            );
            map.insert(
                "next_hop_origin".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_origin,
                )
                .await,
            );
            map.insert(
                "next_hop_vpn_tunnel".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#next_hop_vpn_tunnel,
                )
                .await,
            );
            map.insert(
                "priority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#priority,
                )
                .await,
            );
            map.insert(
                "project".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project,
                )
                .await,
            );
            map.insert(
                "self_link".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#self_link,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RouterStatusBestRoute {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dest_range: {
                        let field_value = match fields_map.get("dest_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'dest_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_gateway: {
                        let field_value = match fields_map.get("next_hop_gateway") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_gateway' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_ilb: {
                        let field_value = match fields_map.get("next_hop_ilb") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_ilb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_instance: {
                        let field_value = match fields_map.get("next_hop_instance") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_instance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_instance_zone: {
                        let field_value = match fields_map.get("next_hop_instance_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_instance_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_inter_region_cost: {
                        let field_value = match fields_map.get("next_hop_inter_region_cost") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_inter_region_cost' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_ip: {
                        let field_value = match fields_map.get("next_hop_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_med: {
                        let field_value = match fields_map.get("next_hop_med") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_med' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_network: {
                        let field_value = match fields_map.get("next_hop_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_origin: {
                        let field_value = match fields_map.get("next_hop_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#next_hop_vpn_tunnel: {
                        let field_value = match fields_map.get("next_hop_vpn_tunnel") {
                            Some(value) => value,
                            None => bail!("Missing field 'next_hop_vpn_tunnel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project: {
                        let field_value = match fields_map.get("project") {
                            Some(value) => value,
                            None => bail!("Missing field 'project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
