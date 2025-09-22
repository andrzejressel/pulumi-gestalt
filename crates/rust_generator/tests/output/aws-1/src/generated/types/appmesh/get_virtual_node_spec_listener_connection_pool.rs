#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecListenerConnectionPool {
    #[builder(into)]
    #[serde(rename = "grpcs")]
    pub r#grpcs: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPoolGrpc>,
    #[builder(into)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPoolHttp2>,
    #[builder(into)]
    #[serde(rename = "https")]
    pub r#https: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPoolHttp>,
    #[builder(into)]
    #[serde(rename = "tcps")]
    pub r#tcps: Vec<super::super::types::appmesh::GetVirtualNodeSpecListenerConnectionPoolTcp>,
}
