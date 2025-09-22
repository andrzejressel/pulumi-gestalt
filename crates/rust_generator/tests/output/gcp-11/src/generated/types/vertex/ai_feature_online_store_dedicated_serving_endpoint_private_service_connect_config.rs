#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureOnlineStoreDedicatedServingEndpointPrivateServiceConnectConfig {
    /// If set to true, customers will use private service connection to send request. Otherwise, the connection will set to public endpoint.
    #[builder(into)]
    #[serde(rename = "enablePrivateServiceConnect")]
    pub r#enable_private_service_connect: bool,
    /// A list of Projects from which the forwarding rule will target the service attachment.
    #[builder(into)]
    #[serde(rename = "projectAllowlists")]
    pub r#project_allowlists: Option<Vec<String>>,
}
