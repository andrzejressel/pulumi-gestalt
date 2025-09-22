#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceEndpointSpecPort {
    /// A random name for the port
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Rrepresents the protocol of a port: `tcp`, `udp` or `sctp`. Defaults to `tcp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
    /// Represents the mode in which the port is to be published: 'ingress' or 'host'. Defaults to `ingress`.
    #[builder(into)]
    #[serde(rename = "publishMode")]
    pub r#publish_mode: Option<String>,
    /// The port on the swarm hosts
    #[builder(into)]
    #[serde(rename = "publishedPort")]
    pub r#published_port: Option<i32>,
    /// The port inside the container
    #[builder(into)]
    #[serde(rename = "targetPort")]
    pub r#target_port: i32,
}
