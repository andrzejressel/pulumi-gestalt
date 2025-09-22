#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolNodePlacement {
    /// The placement policy for allocating nodes in the pool. Values are: "Regional": All nodes in the pool will be allocated in the same region; "Zonal": Nodes in the pool will be spread across different zones with the best effort balancing. Defaults to `Regional`.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
}
