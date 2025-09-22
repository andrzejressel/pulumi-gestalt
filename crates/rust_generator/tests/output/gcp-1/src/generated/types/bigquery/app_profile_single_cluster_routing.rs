#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppProfileSingleClusterRouting {
    /// If true, CheckAndMutateRow and ReadModifyWriteRow requests are allowed by this app profile.
    /// It is unsafe to send these requests to the same table/row/column in multiple clusters.
    #[builder(into)]
    #[serde(rename = "allowTransactionalWrites")]
    pub r#allow_transactional_writes: Option<bool>,
    /// The cluster to which read/write requests should be routed.
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: String,
}
