#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterMigrationSource {
    /// The host and port of the on-premises instance in host:port format
    #[builder(into)]
    #[serde(rename = "hostPort")]
    pub r#host_port: Option<String>,
    /// Place holder for the external source identifier(e.g DMS job name) that created the cluster.
    #[builder(into)]
    #[serde(rename = "referenceId")]
    pub r#reference_id: Option<String>,
    /// Type of migration source.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: Option<String>,
}
