#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTls {
    /// Listener's TLS certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsCertificate>>,
    /// Whether the policy is enforced. Default is `true`.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<bool>,
    /// One or more ports that the policy is enforced for.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<i32>>,
    /// Listener's Transport Layer Security (TLS) validation context.
    #[builder(into)]
    #[serde(rename = "validation")]
    pub r#validation: Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation>,
}
