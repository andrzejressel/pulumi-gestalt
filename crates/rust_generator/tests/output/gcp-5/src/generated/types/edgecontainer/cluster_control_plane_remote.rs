#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterControlPlaneRemote {
    /// Name of the Google Distributed Cloud Edge zones where this node pool
    /// will be created. For example: `us-central1-edge-customer-a`.
    #[builder(into)]
    #[serde(rename = "nodeLocation")]
    pub r#node_location: Option<String>,
}
