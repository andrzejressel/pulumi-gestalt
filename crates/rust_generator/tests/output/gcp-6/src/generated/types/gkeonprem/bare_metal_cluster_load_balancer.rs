#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterLoadBalancer {
    /// Configuration for BGP typed load balancers.
    /// Structure is documented below.
    #[builder(into)]
    pub r#bgp_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerBgpLbConfig>>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    pub r#manual_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerManualLbConfig>>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    pub r#metal_lb_config: Option<Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerMetalLbConfig>>,
    /// Specifies the load balancer ports.
    /// Structure is documented below.
    #[builder(into)]
    pub r#port_config: Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerPortConfig>,
    /// Specified the Bare Metal Load Balancer Config
    /// Structure is documented below.
    #[builder(into)]
    pub r#vip_config: Box<super::super::types::gkeonprem::BareMetalClusterLoadBalancerVipConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterLoadBalancer {
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
                    "bgpLbConfig",
                    &self.r#bgp_lb_config,
                ),
                to_pulumi_object_field(
                    "manualLbConfig",
                    &self.r#manual_lb_config,
                ),
                to_pulumi_object_field(
                    "metalLbConfig",
                    &self.r#metal_lb_config,
                ),
                to_pulumi_object_field(
                    "portConfig",
                    &self.r#port_config,
                ),
                to_pulumi_object_field(
                    "vipConfig",
                    &self.r#vip_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterLoadBalancer {
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
                    r#bgp_lb_config: {
                        let field_value = match fields_map.get("bgpLbConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'bgpLbConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_lb_config: {
                        let field_value = match fields_map.get("manualLbConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'manualLbConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metal_lb_config: {
                        let field_value = match fields_map.get("metalLbConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'metalLbConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_config: {
                        let field_value = match fields_map.get("portConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'portConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vip_config: {
                        let field_value = match fields_map.get("vipConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'vipConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
