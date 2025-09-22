#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeType {
    /// A `application_ports` block as defined below.
    #[builder(into)]
    #[serde(rename = "applicationPorts")]
    pub r#application_ports: Box<Option<super::super::types::servicefabric::ClusterNodeTypeApplicationPorts>>,
    /// The capacity tags applied to the nodes in the node type, the cluster resource manager uses these tags to understand how much resource a node has.
    #[builder(into)]
    #[serde(rename = "capacities")]
    pub r#capacities: Option<std::collections::HashMap<String, String>>,
    /// The Port used for the Client Endpoint for this Node Type.
    #[builder(into)]
    #[serde(rename = "clientEndpointPort")]
    pub r#client_endpoint_port: i32,
    /// The Durability Level for this Node Type. Possible values include `Bronze`, `Gold` and `Silver`. Defaults to `Bronze`.
    #[builder(into)]
    #[serde(rename = "durabilityLevel")]
    pub r#durability_level: Option<String>,
    /// A `ephemeral_ports` block as defined below.
    #[builder(into)]
    #[serde(rename = "ephemeralPorts")]
    pub r#ephemeral_ports: Box<Option<super::super::types::servicefabric::ClusterNodeTypeEphemeralPorts>>,
    /// The Port used for the HTTP Endpoint for this Node Type.
    #[builder(into)]
    #[serde(rename = "httpEndpointPort")]
    pub r#http_endpoint_port: i32,
    /// The number of nodes for this Node Type.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: i32,
    /// Is this the Primary Node Type?
    #[builder(into)]
    #[serde(rename = "isPrimary")]
    pub r#is_primary: bool,
    /// Should this node type run only stateless services?
    #[builder(into)]
    #[serde(rename = "isStateless")]
    pub r#is_stateless: Option<bool>,
    /// Does this node type span availability zones?
    #[builder(into)]
    #[serde(rename = "multipleAvailabilityZones")]
    pub r#multiple_availability_zones: Option<bool>,
    /// The name of the Node Type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The placement tags applied to nodes in the node type, which can be used to indicate where certain services (workload) should run.
    #[builder(into)]
    #[serde(rename = "placementProperties")]
    pub r#placement_properties: Option<std::collections::HashMap<String, String>>,
    /// The Port used for the Reverse Proxy Endpoint for this Node Type. Changing this will upgrade the cluster.
    #[builder(into)]
    #[serde(rename = "reverseProxyEndpointPort")]
    pub r#reverse_proxy_endpoint_port: Option<i32>,
}
