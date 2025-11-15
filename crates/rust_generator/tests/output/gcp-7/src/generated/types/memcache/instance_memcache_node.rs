#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceMemcacheNode {
    /// (Output)
    /// Hostname or IP address of the Memcached node used by the clients to connect to the Memcached server on this node.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// (Output)
    /// Identifier of the Memcached node. The node id does not include project or location like the Memcached instance name.
    #[builder(into)]
    #[serde(rename = "nodeId")]
    pub r#node_id: Option<String>,
    /// (Output)
    /// The port number of the Memcached server on this node.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// (Output)
    /// Current state of the Memcached node.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// (Output)
    /// Location (GCP Zone) for the Memcached node.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Option<String>,
}
