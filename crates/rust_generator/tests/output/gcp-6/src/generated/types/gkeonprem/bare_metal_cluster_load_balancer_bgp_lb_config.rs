#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterLoadBalancerBgpLbConfig {
    /// AddressPools is a list of non-overlapping IP pools used by load balancer
    /// typed services. All addresses must be routable to load balancer nodes.
    /// IngressVIP must be included in the pools.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addressPools")]
    pub r#address_pools: Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigAddressPool>,
    /// BGP autonomous system number (ASN) of the cluster.
    /// This field can be updated after cluster creation.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: i32,
    /// The list of BGP peers that the cluster will connect to.
    /// At least one peer must be configured for each control plane node.
    /// Control plane nodes will connect to these peers to advertise the control
    /// plane VIP. The Services load balancer also uses these peers by default.
    /// This field can be updated after cluster creation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bgpPeerConfigs")]
    pub r#bgp_peer_configs: Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigBgpPeerConfig>,
    /// Specifies the node pool running data plane load balancing. L2 connectivity
    /// is required among nodes in this pool. If missing, the control plane node
    /// pool is used for data plane load balancing.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "loadBalancerNodePoolConfig")]
    pub r#load_balancer_node_pool_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterLoadBalancerBgpLbConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "address_pools",
                    &self.r#address_pools,
                ),
                to_pulumi_object_field(
                    "asn",
                    &self.r#asn,
                ),
                to_pulumi_object_field(
                    "bgp_peer_configs",
                    &self.r#bgp_peer_configs,
                ),
                to_pulumi_object_field(
                    "load_balancer_node_pool_config",
                    &self.r#load_balancer_node_pool_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterLoadBalancerBgpLbConfig {
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
                    r#address_pools: {
                        let field_value = match fields_map.get("address_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#asn: {
                        let field_value = match fields_map.get("asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bgp_peer_configs: {
                        let field_value = match fields_map.get("bgp_peer_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgp_peer_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_node_pool_config: {
                        let field_value = match fields_map.get("load_balancer_node_pool_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_node_pool_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
