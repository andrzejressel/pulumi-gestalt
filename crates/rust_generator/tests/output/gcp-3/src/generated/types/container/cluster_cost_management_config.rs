#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterCostManagementConfig {
    /// Whether to enable the [cost allocation](https://cloud.google.com/kubernetes-engine/docs/how-to/cost-allocations) feature.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}
