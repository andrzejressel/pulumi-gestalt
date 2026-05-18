#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNetworkProfile {
    /// IP address within the Kubernetes service address range that will be used by cluster service discovery (kube-dns). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dnsServiceIp")]
    pub r#dns_service_ip: Option<String>,
    /// Specifies a list of IP versions the Kubernetes Cluster will use to assign IP addresses to its nodes and pods. Possible values are `IPv4` and/or `IPv6`. `IPv4` must always be specified. Changing this forces a new resource to be created.
    /// 
    /// ->**Note:** To configure dual-stack networking `ip_versions` should be set to `["IPv4", "IPv6"]`.
    /// 
    /// ->**Note:** Dual-stack networking requires that the Preview Feature `Microsoft.ContainerService/AKS-EnableDualStack` is enabled and the Resource Provider is re-registered, see [the documentation](https://docs.microsoft.com/azure/aks/configure-kubenet-dual-stack?tabs=azure-cli%2Ckubectl#register-the-aks-enabledualstack-preview-feature) for more information.
    #[builder(into)]
    #[serde(rename = "ipVersions")]
    pub r#ip_versions: Option<Vec<String>>,
    /// A `load_balancer_profile` block as defined below. This can only be specified when `load_balancer_sku` is set to `standard`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancerProfile")]
    pub r#load_balancer_profile: Option<Box<super::super::types::containerservice::KubernetesClusterNetworkProfileLoadBalancerProfile>>,
    /// Specifies the SKU of the Load Balancer used for this Kubernetes Cluster. Possible values are `basic` and `standard`. Defaults to `standard`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancerSku")]
    pub r#load_balancer_sku: Option<String>,
    /// A `nat_gateway_profile` block as defined below. This can only be specified when `load_balancer_sku` is set to `standard` and `outbound_type` is set to `managedNATGateway` or `userAssignedNATGateway`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "natGatewayProfile")]
    pub r#nat_gateway_profile: Option<Box<super::super::types::containerservice::KubernetesClusterNetworkProfileNatGatewayProfile>>,
    /// Specifies the data plane used for building the Kubernetes network. Possible values are `azure` and `cilium`. Defaults to `azure`. Disabling this forces a new resource to be created.
    /// 
    /// > **Note:** When `network_data_plane` is set to `cilium`, the `network_plugin` field can only be set to `azure`.
    /// 
    /// > **Note:** When `network_data_plane` is set to `cilium`, one of either `network_plugin_mode = "overlay"` or `pod_subnet_id` must be specified.
    #[builder(into)]
    #[serde(rename = "networkDataPlane")]
    pub r#network_data_plane: Option<String>,
    /// Network mode to be used with Azure CNI. Possible values are `bridge` and `transparent`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** `network_mode` can only be set to `bridge` for existing Kubernetes Clusters and cannot be used to provision new Clusters - this will be removed by Azure in the future.
    /// 
    /// > **Note:** This property can only be set when `network_plugin` is set to `azure`.
    #[builder(into)]
    #[serde(rename = "networkMode")]
    pub r#network_mode: Option<String>,
    /// Network plugin to use for networking. Currently supported values are `azure`, `kubenet` and `none`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** When `network_plugin` is set to `azure` - the `pod_cidr` field must not be set, unless specifying `network_plugin_mode` to `overlay`.
    #[builder(into)]
    #[serde(rename = "networkPlugin")]
    pub r#network_plugin: String,
    /// Specifies the network plugin mode used for building the Kubernetes network. Possible value is `overlay`.
    /// 
    /// > **Note:** When `network_plugin_mode` is set to `overlay`, the `network_plugin` field can only be set to `azure`. When upgrading from Azure CNI without overlay, `pod_subnet_id` must be specified.
    #[builder(into)]
    #[serde(rename = "networkPluginMode")]
    pub r#network_plugin_mode: Option<String>,
    /// Sets up network policy to be used with Azure CNI. [Network policy allows us to control the traffic flow between pods](https://docs.microsoft.com/azure/aks/use-network-policies). Currently supported values are `calico`, `azure` and `cilium`.
    /// 
    /// > **Note:** When `network_policy` is set to `azure`, the `network_plugin` field can only be set to `azure`.
    /// 
    /// > **Note:** When `network_policy` is set to `cilium`, the `network_data_plane` field must be set to `cilium`.
    #[builder(into)]
    #[serde(rename = "networkPolicy")]
    pub r#network_policy: Option<String>,
    /// The outbound (egress) routing method which should be used for this Kubernetes Cluster. Possible values are `loadBalancer`, `userDefinedRouting`, `managedNATGateway` and `userAssignedNATGateway`. Defaults to `loadBalancer`. More information on supported migration paths for `outbound_type` can be found in [this documentation](https://learn.microsoft.com/azure/aks/egress-outboundtype#updating-outboundtype-after-cluster-creation).
    #[builder(into)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Option<String>,
    /// The CIDR to use for pod IP addresses. This field can only be set when `network_plugin` is set to `kubenet` or `network_plugin_mode` is set to `overlay`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: Option<String>,
    /// A list of CIDRs to use for pod IP addresses. For single-stack networking a single IPv4 CIDR is expected. For dual-stack networking an IPv4 and IPv6 CIDR are expected. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "podCidrs")]
    pub r#pod_cidrs: Option<Vec<String>>,
    /// The Network Range used by the Kubernetes service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: Option<String>,
    /// A list of CIDRs to use for Kubernetes services. For single-stack networking a single IPv4 CIDR is expected. For dual-stack networking an IPv4 and IPv6 CIDR are expected. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This range should not be used by any network element on or connected to this VNet. Service address CIDR must be smaller than /12. `docker_bridge_cidr`, `dns_service_ip` and `service_cidr` should all be empty or all should be set.
    #[builder(into)]
    #[serde(rename = "serviceCidrs")]
    pub r#service_cidrs: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterNetworkProfile {
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
                    "dns_service_ip",
                    &self.r#dns_service_ip,
                ),
                to_pulumi_object_field(
                    "ip_versions",
                    &self.r#ip_versions,
                ),
                to_pulumi_object_field(
                    "load_balancer_profile",
                    &self.r#load_balancer_profile,
                ),
                to_pulumi_object_field(
                    "load_balancer_sku",
                    &self.r#load_balancer_sku,
                ),
                to_pulumi_object_field(
                    "nat_gateway_profile",
                    &self.r#nat_gateway_profile,
                ),
                to_pulumi_object_field(
                    "network_data_plane",
                    &self.r#network_data_plane,
                ),
                to_pulumi_object_field(
                    "network_mode",
                    &self.r#network_mode,
                ),
                to_pulumi_object_field(
                    "network_plugin",
                    &self.r#network_plugin,
                ),
                to_pulumi_object_field(
                    "network_plugin_mode",
                    &self.r#network_plugin_mode,
                ),
                to_pulumi_object_field(
                    "network_policy",
                    &self.r#network_policy,
                ),
                to_pulumi_object_field(
                    "outbound_type",
                    &self.r#outbound_type,
                ),
                to_pulumi_object_field(
                    "pod_cidr",
                    &self.r#pod_cidr,
                ),
                to_pulumi_object_field(
                    "pod_cidrs",
                    &self.r#pod_cidrs,
                ),
                to_pulumi_object_field(
                    "service_cidr",
                    &self.r#service_cidr,
                ),
                to_pulumi_object_field(
                    "service_cidrs",
                    &self.r#service_cidrs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterNetworkProfile {
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
                    r#ip_versions: {
                        let field_value = match fields_map.get("ip_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancer_profile: {
                        let field_value = match fields_map.get("load_balancer_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancer_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#nat_gateway_profile: {
                        let field_value = match fields_map.get("nat_gateway_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'nat_gateway_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_data_plane: {
                        let field_value = match fields_map.get("network_data_plane") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_data_plane' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_mode: {
                        let field_value = match fields_map.get("network_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network_plugin_mode: {
                        let field_value = match fields_map.get("network_plugin_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_plugin_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#outbound_type: {
                        let field_value = match fields_map.get("outbound_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#pod_cidrs: {
                        let field_value = match fields_map.get("pod_cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#service_cidrs: {
                        let field_value = match fields_map.get("service_cidrs") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_cidrs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
