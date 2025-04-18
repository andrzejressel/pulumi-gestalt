#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesCentralServerLoadBalancer {
    /// A list of Backend Pool names for the Load Balancer. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "backendPoolNames")]
    pub r#backend_pool_names: Box<Option<Vec<String>>>,
    /// A list of Frontend IP Configuration names. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "frontendIpConfigurationNames")]
    pub r#frontend_ip_configuration_names: Box<Option<Vec<String>>>,
    /// A list of Health Probe names. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "healthProbeNames")]
    pub r#health_probe_names: Box<Option<Vec<String>>>,
    /// The full resource name of the Load Balancer. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
