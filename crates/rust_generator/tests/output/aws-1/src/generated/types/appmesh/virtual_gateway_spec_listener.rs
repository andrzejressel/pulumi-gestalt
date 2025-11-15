#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpecListener {
    /// Connection pool information for the listener.
    #[builder(into)]
    #[serde(rename = "connectionPool")]
    pub r#connection_pool: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPool>>,
    /// Health check information for the listener.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerHealthCheck>>,
    /// Port mapping information for the listener.
    #[builder(into)]
    #[serde(rename = "portMapping")]
    pub r#port_mapping: Box<super::super::types::appmesh::VirtualGatewaySpecListenerPortMapping>,
    /// Transport Layer Security (TLS) properties for the listener
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerTls>>,
}
