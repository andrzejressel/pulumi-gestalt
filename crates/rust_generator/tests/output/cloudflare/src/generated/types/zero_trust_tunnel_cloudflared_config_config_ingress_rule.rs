#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigIngressRule {
    /// Hostname to match the incoming request with. If the hostname matches, the request will be sent to the service.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    #[builder(into)]
    #[serde(rename = "originRequest")]
    pub r#origin_request: Option<Box<super::types::ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequest>>,
    /// Path of the incoming request. If the path matches, the request will be sent to the local service.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Name of the service to which the request will be sent.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}
