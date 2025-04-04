#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNetworkGatewayVpnClientConfigurationRadiusServer {
    /// The address of the Radius Server.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The score of the Radius Server determines the priority of the server. Possible values are between `1` and `30`.
    #[builder(into)]
    #[serde(rename = "score")]
    pub r#score: Box<i32>,
    /// The secret that is used to communicate with the Radius Server.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
}
