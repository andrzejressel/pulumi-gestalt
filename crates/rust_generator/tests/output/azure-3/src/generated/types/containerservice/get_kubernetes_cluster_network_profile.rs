#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterNetworkProfile {
    /// IP address within the Kubernetes service address range used by cluster service discovery (kube-dns).
    #[builder(into)]
    pub r#dns_service_ip: String,
    /// IP address (in CIDR notation) used as the Docker bridge IP address on nodes.
    #[builder(into)]
    pub r#docker_bridge_cidr: String,
    #[builder(into)]
    pub r#load_balancer_sku: String,
    /// Network plugin used such as `azure` or `kubenet`.
    #[builder(into)]
    pub r#network_plugin: String,
    /// Network policy to be used with Azure CNI. e.g. `calico` or `azure`
    #[builder(into)]
    pub r#network_policy: String,
    /// The CIDR used for pod IP addresses.
    #[builder(into)]
    pub r#pod_cidr: String,
    /// Network range used by the Kubernetes service.
    #[builder(into)]
    pub r#service_cidr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterNetworkProfile {
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
                    "dnsServiceIp",
                    &self.r#dns_service_ip,
                ),
                to_pulumi_object_field(
                    "dockerBridgeCidr",
                    &self.r#docker_bridge_cidr,
                ),
                to_pulumi_object_field(
                    "loadBalancerSku",
                    &self.r#load_balancer_sku,
                ),
                to_pulumi_object_field(
                    "networkPlugin",
                    &self.r#network_plugin,
                ),
                to_pulumi_object_field(
                    "networkPolicy",
                    &self.r#network_policy,
                ),
                to_pulumi_object_field(
                    "podCidr",
                    &self.r#pod_cidr,
                ),
                to_pulumi_object_field(
                    "serviceCidr",
                    &self.r#service_cidr,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetKubernetesClusterNetworkProfile {
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
                    r#dns_service_ip: {
                        let field_value = match fields_map.get("dnsServiceIp") {
                            Some(value) => value,
                            None => bail!("Missing field 'dnsServiceIp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_bridge_cidr: {
                        let field_value = match fields_map.get("dockerBridgeCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerBridgeCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_sku: {
                        let field_value = match fields_map.get("loadBalancerSku") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancerSku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_plugin: {
                        let field_value = match fields_map.get("networkPlugin") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkPlugin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_policy: {
                        let field_value = match fields_map.get("networkPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'networkPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_cidr: {
                        let field_value = match fields_map.get("podCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'podCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_cidr: {
                        let field_value = match fields_map.get("serviceCidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceCidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
