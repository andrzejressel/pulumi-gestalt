#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InteractiveQueryClusterRoles {
    /// A `head_node` block as defined above.
    #[builder(into)]
    #[serde(rename = "headNode")]
    pub r#head_node: Box<super::super::types::hdinsight::InteractiveQueryClusterRolesHeadNode>,
    /// A `worker_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "workerNode")]
    pub r#worker_node: Box<super::super::types::hdinsight::InteractiveQueryClusterRolesWorkerNode>,
    /// A `zookeeper_node` block as defined below.
    #[builder(into)]
    #[serde(rename = "zookeeperNode")]
    pub r#zookeeper_node: Box<super::super::types::hdinsight::InteractiveQueryClusterRolesZookeeperNode>,
}
