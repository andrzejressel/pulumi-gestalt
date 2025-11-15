#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDomainEndpointOptions {
    /// Fully qualified domain for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpoint")]
    pub r#custom_endpoint: Option<String>,
    /// ACM certificate ARN for your custom endpoint.
    #[builder(into)]
    #[serde(rename = "customEndpointCertificateArn")]
    pub r#custom_endpoint_certificate_arn: Option<String>,
    /// Whether to enable custom endpoint for the OpenSearch domain.
    #[builder(into)]
    #[serde(rename = "customEndpointEnabled")]
    pub r#custom_endpoint_enabled: Option<bool>,
    /// Whether or not to require HTTPS. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enforceHttps")]
    pub r#enforce_https: Option<bool>,
    /// Name of the TLS security policy that needs to be applied to the HTTPS endpoint. For valid values, refer to the [AWS documentation](https://docs.aws.amazon.com/opensearch-service/latest/APIReference/API_DomainEndpointOptions.html#opensearchservice-Type-DomainEndpointOptions-TLSSecurityPolicy). Pulumi will only perform drift detection if a configuration value is provided.
    #[builder(into)]
    #[serde(rename = "tlsSecurityPolicy")]
    pub r#tls_security_policy: Option<String>,
}
