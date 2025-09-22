#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecListenerTlsCertificate {
    /// An AWS Certificate Manager (ACM) certificate.
    #[builder(into)]
    #[serde(rename = "acm")]
    pub r#acm: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTlsCertificateAcm>>,
    /// Local file certificate.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTlsCertificateFile>>,
    /// A [Secret Discovery Service](https://www.envoyproxy.io/docs/envoy/latest/configuration/security/secret#secret-discovery-service-sds) certificate.
    #[builder(into)]
    #[serde(rename = "sds")]
    pub r#sds: Option<Box<super::super::types::appmesh::VirtualNodeSpecListenerTlsCertificateSds>>,
}
