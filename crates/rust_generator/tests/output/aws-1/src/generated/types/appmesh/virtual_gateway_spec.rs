#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpec {
    /// Defaults for backends.
    #[builder(into)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaults>>,
    /// Listeners that the mesh endpoint is expected to receive inbound traffic from. You can specify one listener.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Vec<super::super::types::appmesh::VirtualGatewaySpecListener>,
    /// Inbound and outbound access logging information for the virtual gateway.
    #[builder(into)]
    #[serde(rename = "logging")]
    pub r#logging: Option<Box<super::super::types::appmesh::VirtualGatewaySpecLogging>>,
}
