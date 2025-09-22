#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpec {
    /// Defaults for backends.
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaults>>,
    /// Backends to which the virtual node is expected to send outbound traffic.
    #[builder(into)]
    #[serde(rename = "backends")]
    pub r#backends: Option<Vec<super::super::types::appmesh::VirtualNodeSpecBackend>>,
    /// Listeners from which the virtual node is expected to receive inbound traffic.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Option<Vec<super::super::types::appmesh::VirtualNodeSpecListener>>,
    /// Inbound and outbound access logging information for the virtual node.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::appmesh::VirtualNodeSpecLogging>>,
    /// Service discovery information for the virtual node.
    #[builder(into)]
    #[serde(rename = "serviceDiscovery")]
    pub r#service_discovery: Option<Box<super::super::types::appmesh::VirtualNodeSpecServiceDiscovery>>,
}
