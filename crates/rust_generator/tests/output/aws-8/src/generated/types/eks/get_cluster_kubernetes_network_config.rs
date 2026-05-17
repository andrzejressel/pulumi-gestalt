#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterKubernetesNetworkConfig {
    /// Contains Elastic Load Balancing configuration for EKS Auto Mode enabled cluster.
    #[builder(into)]
    #[serde(rename = "elasticLoadBalancings")]
    pub r#elastic_load_balancings: Vec<super::super::types::eks::GetClusterKubernetesNetworkConfigElasticLoadBalancing>,
    /// `ipv4` or `ipv6`.
    #[builder(into)]
    #[serde(rename = "ipFamily")]
    pub r#ip_family: String,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from if `ipv4` was specified when the cluster was created.
    #[builder(into)]
    #[serde(rename = "serviceIpv4Cidr")]
    pub r#service_ipv_4_cidr: String,
    /// The CIDR block to assign Kubernetes pod and service IP addresses from if `ipv6` was specified when the cluster was created. Kubernetes assigns service addresses from the unique local address range (fc00::/7) because you can't specify a custom IPv6 CIDR block when you create the cluster.
    #[builder(into)]
    #[serde(rename = "serviceIpv6Cidr")]
    pub r#service_ipv_6_cidr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetClusterKubernetesNetworkConfig {
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
                "elastic_load_balancings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elastic_load_balancings,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetClusterKubernetesNetworkConfig {
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
                    r#elastic_load_balancings: {
                        let field_value = match fields_map.get("elastic_load_balancings") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastic_load_balancings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
