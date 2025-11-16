#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterLoadBalancerManualLbConfig {
    /// NodePort for control plane service. The Kubernetes API server in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30968).
    #[builder(into)]
    #[serde(rename = "controlPlaneNodePort")]
    pub r#control_plane_node_port: Option<i32>,
    /// NodePort for ingress service's http. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 32527).
    #[builder(into)]
    #[serde(rename = "ingressHttpNodePort")]
    pub r#ingress_http_node_port: Option<i32>,
    /// NodePort for ingress service's https. The ingress service in the admin
    /// cluster is implemented as a Service of type NodePort (ex. 30139).
    #[builder(into)]
    #[serde(rename = "ingressHttpsNodePort")]
    pub r#ingress_https_node_port: Option<i32>,
    /// NodePort for konnectivity server service running as a sidecar in each
    /// kube-apiserver pod (ex. 30564).
    #[builder(into)]
    #[serde(rename = "konnectivityServerNodePort")]
    pub r#konnectivity_server_node_port: Option<i32>,
}
