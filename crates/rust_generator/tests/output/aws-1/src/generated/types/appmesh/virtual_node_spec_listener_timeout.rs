#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecListenerTimeout {
    /// Timeouts for gRPC listeners.
    #[builder(into)]
    #[serde(rename = "grpc")]
    pub r#grpc: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutGrpc>>,
    /// Timeouts for HTTP listeners.
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp>>,
    /// Timeouts for HTTP2 listeners.
    #[builder(into)]
    #[serde(rename = "http2")]
    pub r#http_2: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutHttp2>>,
    /// Timeouts for TCP listeners.
    #[builder(into)]
    #[serde(rename = "tcp")]
    pub r#tcp: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTimeoutTcp>>,
}
