#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterCacheNode {
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: String,
    /// Availability Zone for the cache cluster.
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: String,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    #[builder(into)]
    #[serde(rename = "outpostArn")]
    pub r#outpost_arn: String,
    /// The port number on which each of the cache nodes will
    /// accept connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
