#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiFeatureOnlineStoreDedicatedServingEndpoint {
    /// Private service connect config.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServiceConnectConfig")]
    pub r#private_service_connect_config: Option<Box<super::super::types::vertex::AiFeatureOnlineStoreDedicatedServingEndpointPrivateServiceConnectConfig>>,
    /// (Output)
    /// Domain name to use for this FeatureOnlineStore
    #[builder(into)]
    #[serde(rename = "publicEndpointDomainName")]
    pub r#public_endpoint_domain_name: Option<String>,
    /// (Output)
    /// Name of the service attachment resource. Applicable only if private service connect is enabled and after FeatureViewSync is created.
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}
