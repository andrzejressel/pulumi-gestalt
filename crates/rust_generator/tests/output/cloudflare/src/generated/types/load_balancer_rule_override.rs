#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LoadBalancerRuleOverride {
    /// Controls features that modify the routing of requests to pools and origins in response to dynamic conditions, such as during the interval between active health monitoring requests.
    #[builder(into)]
    #[serde(rename = "adaptiveRoutings")]
    pub r#adaptive_routings: Option<Vec<super::types::LoadBalancerRuleOverrideAdaptiveRouting>>,
    /// A set containing mappings of country codes to a list of pool IDs (ordered by their failover priority) for the given country.
    #[builder(into)]
    #[serde(rename = "countryPools")]
    pub r#country_pools: Option<Vec<super::types::LoadBalancerRuleOverrideCountryPool>>,
    /// A list of pool IDs ordered by their failover priority. Used whenever `pop_pools`/`country_pools`/`region_pools` are not defined.
    #[builder(into)]
    #[serde(rename = "defaultPools")]
    pub r#default_pools: Option<Vec<String>>,
    /// The pool ID to use when all other pools are detected as unhealthy.
    #[builder(into)]
    #[serde(rename = "fallbackPool")]
    pub r#fallback_pool: Option<String>,
    /// Controls location-based steering for non-proxied requests.
    #[builder(into)]
    #[serde(rename = "locationStrategies")]
    pub r#location_strategies: Option<Vec<super::types::LoadBalancerRuleOverrideLocationStrategy>>,
    /// A set containing mappings of Cloudflare Point-of-Presence (PoP) identifiers to a list of pool IDs (ordered by their failover priority) for the PoP (datacenter). This feature is only available to enterprise customers.
    #[builder(into)]
    #[serde(rename = "popPools")]
    pub r#pop_pools: Option<Vec<super::types::LoadBalancerRuleOverridePopPool>>,
    /// Configures pool weights. When `steering_policy="random"`, a random pool is selected with probability proportional to pool weights. When `steering_policy="least_outstanding_requests"`, pool weights are used to scale each pool's outstanding requests. When `steering_policy="least_connections"`, pool weights are used to scale each pool's open connections.
    #[builder(into)]
    #[serde(rename = "randomSteerings")]
    pub r#random_steerings: Option<Vec<super::types::LoadBalancerRuleOverrideRandomSteering>>,
    /// A set containing mappings of region codes to a list of pool IDs (ordered by their failover priority) for the given region.
    #[builder(into)]
    #[serde(rename = "regionPools")]
    pub r#region_pools: Option<Vec<super::types::LoadBalancerRuleOverrideRegionPool>>,
    /// Configure attributes for session affinity.
    #[builder(into)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Option<String>,
    /// Configure attributes for session affinity. Note that the property `drain_duration` is not currently supported as a rule override.
    #[builder(into)]
    #[serde(rename = "sessionAffinityAttributes")]
    pub r#session_affinity_attributes: Option<Vec<super::types::LoadBalancerRuleOverrideSessionAffinityAttribute>>,
    /// Time, in seconds, until this load balancer's session affinity cookie expires after being created. This parameter is ignored unless a supported session affinity policy is set. The current default of `82800` (23 hours) will be used unless `session_affinity_ttl` is explicitly set. Once the expiry time has been reached, subsequent requests may get sent to a different origin server. Valid values are between `1800` and `604800`.
    #[builder(into)]
    #[serde(rename = "sessionAffinityTtl")]
    pub r#session_affinity_ttl: Option<i32>,
    /// The method the load balancer uses to determine the route to your origin. Value `off` uses `default_pool_ids`. Value `geo` uses `pop_pools`/`country_pools`/`region_pools`. For non-proxied requests, the `country` for `country_pools` is determined by `location_strategy`. Value `random` selects a pool randomly. Value `dynamic_latency` uses round trip time to select the closest pool in `default_pool_ids` (requires pool health checks). Value `proximity` uses the pools' latitude and longitude to select the closest pool using the Cloudflare PoP location for proxied requests or the location determined by `location_strategy` for non-proxied requests. Value `least_outstanding_requests` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of outstanding requests. Pools with more pending requests are weighted proportionately less relative to others. Value `least_connections` selects a pool by taking into consideration `random_steering` weights, as well as each pool's number of open connections. Pools with more open connections are weighted proportionately less relative to others. Supported for HTTP/1 and HTTP/2 connections. Value `""` maps to `geo` if you use `pop_pools`/`country_pools`/`region_pools` otherwise `off`. Available values: `off`, `geo`, `dynamic_latency`, `random`, `proximity`, `least_outstanding_requests`, `least_connections`, `""` Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "steeringPolicy")]
    pub r#steering_policy: Option<String>,
    /// Time to live (TTL) of the DNS entry for the IP address returned by this load balancer. This cannot be set for proxied load balancers. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LoadBalancerRuleOverride {
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
                "adaptive_routings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#adaptive_routings,
                )
                .await,
            );
            map.insert(
                "country_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#country_pools,
                )
                .await,
            );
            map.insert(
                "default_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_pools,
                )
                .await,
            );
            map.insert(
                "fallback_pool".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fallback_pool,
                )
                .await,
            );
            map.insert(
                "location_strategies".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location_strategies,
                )
                .await,
            );
            map.insert(
                "pop_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pop_pools,
                )
                .await,
            );
            map.insert(
                "random_steerings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#random_steerings,
                )
                .await,
            );
            map.insert(
                "region_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#region_pools,
                )
                .await,
            );
            map.insert(
                "session_affinity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_affinity,
                )
                .await,
            );
            map.insert(
                "session_affinity_attributes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_affinity_attributes,
                )
                .await,
            );
            map.insert(
                "session_affinity_ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#session_affinity_ttl,
                )
                .await,
            );
            map.insert(
                "steering_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#steering_policy,
                )
                .await,
            );
            map.insert(
                "ttl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ttl,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LoadBalancerRuleOverride {
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
                    r#adaptive_routings: {
                        let field_value = match fields_map.get("adaptive_routings") {
                            Some(value) => value,
                            None => bail!("Missing field 'adaptive_routings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country_pools: {
                        let field_value = match fields_map.get("country_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'country_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_pools: {
                        let field_value = match fields_map.get("default_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fallback_pool: {
                        let field_value = match fields_map.get("fallback_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'fallback_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location_strategies: {
                        let field_value = match fields_map.get("location_strategies") {
                            Some(value) => value,
                            None => bail!("Missing field 'location_strategies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pop_pools: {
                        let field_value = match fields_map.get("pop_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'pop_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#random_steerings: {
                        let field_value = match fields_map.get("random_steerings") {
                            Some(value) => value,
                            None => bail!("Missing field 'random_steerings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region_pools: {
                        let field_value = match fields_map.get("region_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'region_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity: {
                        let field_value = match fields_map.get("session_affinity") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity_attributes: {
                        let field_value = match fields_map.get("session_affinity_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_affinity_ttl: {
                        let field_value = match fields_map.get("session_affinity_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_affinity_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#steering_policy: {
                        let field_value = match fields_map.get("steering_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'steering_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttl: {
                        let field_value = match fields_map.get("ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
