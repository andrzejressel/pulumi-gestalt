#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterLoadBalancerMetalLbConfig {
    /// AddressPools is a list of non-overlapping IP pools used by load balancer
    /// typed services. All addresses must be routable to load balancer nodes.
    /// IngressVIP must be included in the pools.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "addressPools")]
    pub r#address_pools: Vec<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfigAddressPool>,
    /// Specifies the load balancer's node pool configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "loadBalancerNodePoolConfig")]
    pub r#load_balancer_node_pool_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfigLoadBalancerNodePoolConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterLoadBalancerMetalLbConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "address_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_pools,
                )
                .await,
            );
            map.insert(
                "load_balancer_node_pool_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancer_node_pool_config,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterLoadBalancerMetalLbConfig {
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
