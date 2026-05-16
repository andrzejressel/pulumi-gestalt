#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PoolNodePlacement {
    /// The placement policy for allocating nodes in the pool. Values are: "Regional": All nodes in the pool will be allocated in the same region; "Zonal": Nodes in the pool will be spread across different zones with the best effort balancing. Defaults to `Regional`.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
}
