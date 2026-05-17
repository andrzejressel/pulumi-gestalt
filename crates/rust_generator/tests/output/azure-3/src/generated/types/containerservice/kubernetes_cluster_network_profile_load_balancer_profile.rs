#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNetworkProfileLoadBalancerProfile {
    /// The type of the managed inbound Load Balancer Backend Pool. Possible values are `NodeIP` and `NodeIPConfiguration`. Defaults to `NodeIPConfiguration`. See [the documentation](https://learn.microsoft.com/en-us/azure/aks/load-balancer-standard#change-the-inbound-pool-type) for more information.
    #[builder(into)]
    #[serde(rename = "backendPoolType")]
    pub r#backend_pool_type: Option<String>,
    /// The outcome (resource IDs) of the specified arguments.
    #[builder(into)]
    #[serde(rename = "effectiveOutboundIps")]
    pub r#effective_outbound_ips: Option<Vec<String>>,
    /// Desired outbound flow idle timeout in minutes for the cluster load balancer. Must be between `4` and `100` inclusive. Defaults to `30`.
    #[builder(into)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Option<i32>,
    /// Count of desired managed outbound IPs for the cluster load balancer. Must be between `1` and `100` inclusive.
    #[builder(into)]
    #[serde(rename = "managedOutboundIpCount")]
    pub r#managed_outbound_ip_count: Option<i32>,
    /// The desired number of IPv6 outbound IPs created and managed by Azure for the cluster load balancer. Must be in the range of 1 to 100 (inclusive). The default value is 0 for single-stack and 1 for dual-stack.
    /// 
    /// > **Note:** `managed_outbound_ipv6_count` requires dual-stack networking. To enable dual-stack networking the Preview Feature `Microsoft.ContainerService/AKS-EnableDualStack` needs to be enabled and the Resource Provider re-registered, see [the documentation](https://docs.microsoft.com/azure/aks/configure-kubenet-dual-stack?tabs=azure-cli%2Ckubectl#register-the-aks-enabledualstack-preview-feature) for more information.
    #[builder(into)]
    #[serde(rename = "managedOutboundIpv6Count")]
    pub r#managed_outbound_ipv_6_count: Option<i32>,
    /// The ID of the Public IP Addresses which should be used for outbound communication for the cluster load balancer.
    /// 
    /// > **Note:** Set `outbound_ip_address_ids` to an empty slice `[]` in order to unlink it from the cluster. Unlinking a `outbound_ip_address_ids` will revert the load balancing for the cluster back to a managed one.
    #[builder(into)]
    #[serde(rename = "outboundIpAddressIds")]
    pub r#outbound_ip_address_ids: Option<Vec<String>>,
    /// The ID of the outbound Public IP Address Prefixes which should be used for the cluster load balancer.
    /// 
    /// > **Note:** Set `outbound_ip_prefix_ids` to an empty slice `[]` in order to unlink it from the cluster. Unlinking a `outbound_ip_prefix_ids` will revert the load balancing for the cluster back to a managed one.
    #[builder(into)]
    #[serde(rename = "outboundIpPrefixIds")]
    pub r#outbound_ip_prefix_ids: Option<Vec<String>>,
    /// Number of desired SNAT port for each VM in the clusters load balancer. Must be between `0` and `64000` inclusive. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "outboundPortsAllocated")]
    pub r#outbound_ports_allocated: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterNetworkProfileLoadBalancerProfile {
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
                "backend_pool_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#backend_pool_type,
                )
                .await,
            );
            map.insert(
                "effective_outbound_ips".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#effective_outbound_ips,
                )
                .await,
            );
            map.insert(
                "idle_timeout_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idle_timeout_in_minutes,
                )
                .await,
            );
            map.insert(
                "managed_outbound_ip_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_outbound_ip_count,
                )
                .await,
            );
            map.insert(
                "managed_outbound_ipv_6_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_outbound_ipv_6_count,
                )
                .await,
            );
            map.insert(
                "outbound_ip_address_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outbound_ip_address_ids,
                )
                .await,
            );
            map.insert(
                "outbound_ip_prefix_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outbound_ip_prefix_ids,
                )
                .await,
            );
            map.insert(
                "outbound_ports_allocated".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outbound_ports_allocated,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterNetworkProfileLoadBalancerProfile {
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
                    r#backend_pool_type: {
                        let field_value = match fields_map.get("backend_pool_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_pool_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_outbound_ips: {
                        let field_value = match fields_map.get("effective_outbound_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_outbound_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("idle_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_outbound_ip_count: {
                        let field_value = match fields_map.get("managed_outbound_ip_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_outbound_ip_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_outbound_ipv_6_count: {
                        let field_value = match fields_map.get("managed_outbound_ipv_6_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_outbound_ipv_6_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_ip_address_ids: {
                        let field_value = match fields_map.get("outbound_ip_address_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_ip_address_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_ip_prefix_ids: {
                        let field_value = match fields_map.get("outbound_ip_prefix_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_ip_prefix_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outbound_ports_allocated: {
                        let field_value = match fields_map.get("outbound_ports_allocated") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_ports_allocated' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
