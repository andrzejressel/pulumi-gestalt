#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerTimeoutGrpc {
    /// Idle timeout. An idle timeout bounds the amount of time that a connection may be idle.
    #[builder(into)]
    #[serde(rename = "idle")]
    pub r#idle: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutGrpcIdle>>,
    /// Per request timeout.
    #[builder(into)]
    #[serde(rename = "perRequest")]
    pub r#per_request: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutGrpcPerRequest>>,
}
