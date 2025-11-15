#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AiEndpointPrivateServiceConnectConfig {
    /// Required. If true, expose the IndexEndpoint via private service connect.
    #[builder(into)]
    #[serde(rename = "enablePrivateServiceConnect")]
    pub r#enable_private_service_connect: bool,
    /// If set to true, enable secure private service connect with IAM authorization. Otherwise, private service connect will be done without authorization. Note latency will be slightly increased if authorization is enabled.
    #[builder(into)]
    #[serde(rename = "enableSecurePrivateServiceConnect")]
    pub r#enable_secure_private_service_connect: Option<bool>,
    /// A list of Projects from which the forwarding rule will target the service attachment.
    #[builder(into)]
    #[serde(rename = "projectAllowlists")]
    pub r#project_allowlists: Option<Vec<String>>,
}
