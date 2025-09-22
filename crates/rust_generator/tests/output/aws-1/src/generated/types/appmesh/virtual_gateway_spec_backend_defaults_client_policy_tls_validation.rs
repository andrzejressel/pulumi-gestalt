#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidation {
    /// SANs for a virtual gateway's listener's Transport Layer Security (TLS) validation context.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Option<Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationSubjectAlternativeNames>>,
    /// TLS validation context trust.
    #[builder(into)]
    #[serde(rename = "trust")]
    pub r#trust: Box<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicyTlsValidationTrust>,
}
