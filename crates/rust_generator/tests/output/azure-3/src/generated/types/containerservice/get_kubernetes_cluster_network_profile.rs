#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
