#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterShardNodeEndpoint {
    /// DNS hostname of the node.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// The port number on which each of the nodes accepts connections. Defaults to `6379`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
