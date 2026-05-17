#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterNetworkProfile {
    /// IP address within the Kubernetes service address range used by cluster service discovery (kube-dns).
    #[builder(into)]
    #[serde(rename = "dnsServiceIp")]
    pub r#dns_service_ip: String,
    /// IP address (in CIDR notation) used as the Docker bridge IP address on nodes.
    #[builder(into)]
    #[serde(rename = "dockerBridgeCidr")]
    pub r#docker_bridge_cidr: String,
    #[builder(into)]
    #[serde(rename = "loadBalancerSku")]
    pub r#load_balancer_sku: String,
    /// Network plugin used such as `azure` or `kubenet`.
    #[builder(into)]
    #[serde(rename = "networkPlugin")]
    pub r#network_plugin: String,
    /// Network policy to be used with Azure CNI. e.g. `calico` or `azure`
    #[builder(into)]
    #[serde(rename = "networkPolicy")]
    pub r#network_policy: String,
    /// The CIDR used for pod IP addresses.
    #[builder(into)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: String,
    /// Network range used by the Kubernetes service.
    #[builder(into)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetKubernetesClusterNetworkProfile {
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
                    "dns_service_ip",
                    &self.r#dns_service_ip,
                ),
                to_pulumi_object_field(
                    "docker_bridge_cidr",
                    &self.r#docker_bridge_cidr,
                ),
                to_pulumi_object_field(
                    "load_balancer_sku",
                    &self.r#load_balancer_sku,
                ),
                to_pulumi_object_field(
                    "network_plugin",
                    &self.r#network_plugin,
                ),
                to_pulumi_object_field(
                    "network_policy",
                    &self.r#network_policy,
                ),
                to_pulumi_object_field(
                    "pod_cidr",
                    &self.r#pod_cidr,
                ),
                to_pulumi_object_field(
                    "service_cidr",
                    &self.r#service_cidr,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("dns_service_ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'dns_service_ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_bridge_cidr: {
                        let field_value = match fields_map.get("docker_bridge_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_bridge_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_sku: {
                        let field_value = match fields_map.get("load_balancer_sku") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_sku' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_plugin: {
                        let field_value = match fields_map.get("network_plugin") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_plugin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_policy: {
                        let field_value = match fields_map.get("network_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_cidr: {
                        let field_value = match fields_map.get("pod_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_cidr: {
                        let field_value = match fields_map.get("service_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
