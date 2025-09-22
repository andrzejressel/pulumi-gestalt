#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecListenerTimeoutGrpc {
    #[builder(into)]
    #[serde(rename = "idles")]
    pub r#idles: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutGrpcIdle>,
    #[builder(into)]
    #[serde(rename = "perRequests")]
    pub r#per_requests: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerTimeoutGrpcPerRequest>,
}
