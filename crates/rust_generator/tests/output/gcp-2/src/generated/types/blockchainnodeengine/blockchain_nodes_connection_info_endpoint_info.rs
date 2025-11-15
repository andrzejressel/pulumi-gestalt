#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BlockchainNodesConnectionInfoEndpointInfo {
    /// (Output)
    /// The assigned URL for the node JSON-RPC API endpoint.
    #[builder(into)]
    #[serde(rename = "jsonRpcApiEndpoint")]
    pub r#json_rpc_api_endpoint: Option<String>,
    /// (Output)
    /// The assigned URL for the node WebSockets API endpoint.
    #[builder(into)]
    #[serde(rename = "websocketsApiEndpoint")]
    pub r#websockets_api_endpoint: Option<String>,
}
