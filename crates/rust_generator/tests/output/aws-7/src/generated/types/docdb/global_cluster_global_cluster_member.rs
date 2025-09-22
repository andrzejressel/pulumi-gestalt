#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GlobalClusterGlobalClusterMember {
    /// Amazon Resource Name (ARN) of member DB Cluster.
    #[builder(into)]
    #[serde(rename = "dbClusterArn")]
    pub r#db_cluster_arn: Option<String>,
    /// Whether the member is the primary DB Cluster.
    #[builder(into)]
    #[serde(rename = "isWriter")]
    pub r#is_writer: Option<bool>,
}
