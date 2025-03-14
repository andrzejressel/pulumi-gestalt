#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterEnterpriseConfig {
    /// The effective tier of the cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterTier")]
    pub r#cluster_tier: Box<Option<String>>,
    /// Sets the tier of the cluster. Available options include `STANDARD` and `ENTERPRISE`.
    #[builder(into, default)]
    #[serde(rename = "desiredTier")]
    pub r#desired_tier: Box<Option<String>>,
}
