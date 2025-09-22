#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecBackendDefaultsClientPolicy {
    /// Transport Layer Security (TLS) client policy.
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<Box<super::super::types::appmesh::VirtualNodeSpecBackendDefaultsClientPolicyTls>>,
}
