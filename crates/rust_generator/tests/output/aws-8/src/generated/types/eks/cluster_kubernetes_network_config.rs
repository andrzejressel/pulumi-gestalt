#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterKubernetesNetworkConfig {
    /// Configuration block with elastic load balancing configuration for the cluster. Detailed below.
    #[builder(into)]
    #[serde(rename = "elasticLoadBalancing")]
    pub r#elastic_load_balancing: Option<Box<super::super::types::eks::ClusterKubernetesNetworkConfigElasticLoadBalancing>>,
    /// The IP family used to assign Kubernetes pod and service addresses. Valid values are `ipv4` (default) and `ipv6`. You can only specify an IP family when you create a cluster, changing this value will force a new cluster to be created.
    #[builder(into)]
    #[serde(rename = "ipFamily")]
    pub r#ip_family: Option<String>,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from. If you don't specify a block, Kubernetes assigns addresses from either the 10.100.0.0/16 or 172.20.0.0/16 CIDR blocks. We recommend that you specify a block that does not overlap with resources in other networks that are peered or connected to your VPC. You can only specify a custom CIDR block when you create a cluster, changing this value will force a new cluster to be created. The block must meet the following requirements:
    /// 
    /// * Within one of the following private IP address blocks: 10.0.0.0/8, 172.16.0.0/12, or 192.168.0.0/16.
    /// 
    /// * Doesn't overlap with any CIDR block assigned to the VPC that you selected for VPC.
    /// 
    /// * Between /24 and /12.
    #[builder(into)]
    #[serde(rename = "serviceIpv4Cidr")]
    pub r#service_ipv_4_cidr: Option<String>,
    /// The CIDR block that Kubernetes pod and service IP addresses are assigned from if you specified `ipv6` for `ip_family` when you created the cluster. Kubernetes assigns service addresses from the unique local address range (fc00::/7) because you can't specify a custom IPv6 CIDR block when you create the cluster.
    #[builder(into)]
    #[serde(rename = "serviceIpv6Cidr")]
    pub r#service_ipv_6_cidr: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterKubernetesNetworkConfig {
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
                "elastic_load_balancing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elastic_load_balancing,
                )
                .await,
            );
            map.insert(
                "ip_family".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_family,
                )
                .await,
            );
            map.insert(
                "service_ipv_4_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_ipv_4_cidr,
                )
                .await,
            );
            map.insert(
                "service_ipv_6_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_ipv_6_cidr,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterKubernetesNetworkConfig {
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
                    r#elastic_load_balancing: {
                        let field_value = match fields_map.get("elastic_load_balancing") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastic_load_balancing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_family: {
                        let field_value = match fields_map.get("ip_family") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_family' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_ipv_4_cidr: {
                        let field_value = match fields_map.get("service_ipv_4_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_ipv_4_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_ipv_6_cidr: {
                        let field_value = match fields_map.get("service_ipv_6_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_ipv_6_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
