#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecListenerTlsValidationTrust {
    /// TLS validation context trust for a local file certificate.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerTlsValidationTrustFile>>,
    /// TLS validation context trust for a [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Option<Box<super::super::types::appmesh::VirtualGatewaySpecListenerTlsValidationTrustSds>>,
}
