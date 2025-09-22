#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecListenerTls {
    /// Listener's TLS certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Box<super::super::types::appmesh::VirtualGatewaySpecListenerTlsCertificate>,
    /// Listener's TLS mode. Valid values: `DISABLED`, `PERMISSIVE`, `STRICT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// Listener's Transport Layer Security (TLS) validation context.
    #[builder(into)]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<super::super::types::appmesh::VirtualGatewaySpecListenerTlsValidation>>,
}
