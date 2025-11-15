#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterClusterConfigEndpointConfig {
    /// The flag to enable http access to specific ports
    /// on the cluster from external sources (aka Component Gateway). Defaults to false.
    #[builder(into)]
    #[serde(rename = "enableHttpPortAccess")]
    pub r#enable_http_port_access: bool,
    /// The map of port descriptions to URLs. Will only be populated if
    /// `enable_http_port_access` is true.
    #[builder(into)]
    #[serde(rename = "httpPorts")]
    pub r#http_ports: Option<std::collections::HashMap<String, String>>,
}
