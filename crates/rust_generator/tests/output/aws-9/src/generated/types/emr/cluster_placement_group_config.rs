#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterPlacementGroupConfig {
    /// Role of the instance in the cluster. Valid Values: `MASTER`, `CORE`, `TASK`.
    #[builder(into)]
    #[serde(rename = "instanceRole")]
    pub r#instance_role: String,
    /// EC2 Placement Group strategy associated with instance role. Valid Values: `SPREAD`, `PARTITION`, `CLUSTER`, `NONE`.
    #[builder(into)]
    #[serde(rename = "placementStrategy")]
    pub r#placement_strategy: Option<String>,
}
