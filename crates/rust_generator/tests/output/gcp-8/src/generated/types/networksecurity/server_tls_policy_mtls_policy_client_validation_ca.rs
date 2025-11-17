#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerTlsPolicyMtlsPolicyClientValidationCa {
    /// Optional if policy is to be used with Traffic Director. For external HTTPS load balancer must be empty.
    /// Defines a mechanism to provision server identity (public and private keys). Cannot be combined with allowOpen as a permissive mode that allows both plain text and TLS is not supported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "certificateProviderInstance")]
    pub r#certificate_provider_instance: Option<Box<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicyClientValidationCaCertificateProviderInstance>>,
    /// gRPC specific configuration to access the gRPC server to obtain the cert and private key.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "grpcEndpoint")]
    pub r#grpc_endpoint: Option<Box<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicyClientValidationCaGrpcEndpoint>>,
}
