#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualGatewaySpecListener {
    #[builder(into)]
    #[serde(rename = "connectionPools")]
    pub r#connection_pools: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerConnectionPool>,
    #[builder(into)]
    #[serde(rename = "healthChecks")]
    pub r#health_checks: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerHealthCheck>,
    #[builder(into)]
    #[serde(rename = "portMappings")]
    pub r#port_mappings: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerPortMapping>,
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Vec<super::super::types::appmesh::GetVirtualGatewaySpecListenerTl>,
}
