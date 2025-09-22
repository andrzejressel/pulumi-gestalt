#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionAutoscalerAutoscalingPolicyScaleDownControlMaxScaledDownReplicas {
    /// Specifies a fixed number of VM instances. This must be a positive
    /// integer.
    #[builder(into)]
    #[serde(rename = "fixed")]
    pub r#fixed: Option<i32>,
    /// Specifies a percentage of instances between 0 to 100%, inclusive.
    /// For example, specify 80 for 80%.
    #[builder(into)]
    #[serde(rename = "percent")]
    pub r#percent: Option<i32>,
}
