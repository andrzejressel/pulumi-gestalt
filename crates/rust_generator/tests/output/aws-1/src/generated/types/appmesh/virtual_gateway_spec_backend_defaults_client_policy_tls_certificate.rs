#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate {
    /// Local file certificate.
    #[builder(into, default)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateFile>>,
    /// A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
    #[builder(into, default)]
    #[serde(rename = "sds")]
    pub r#sds: Box<Option<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificateSds>>,
}
