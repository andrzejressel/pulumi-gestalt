#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterShardNodeEndpoint {
    /// DNS hostname of the node.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// Port number that this node is listening on.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
