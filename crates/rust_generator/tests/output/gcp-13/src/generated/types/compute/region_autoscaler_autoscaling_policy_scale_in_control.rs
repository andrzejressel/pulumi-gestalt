#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionAutoscalerAutoscalingPolicyScaleInControl {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maxScaledInReplicas")]
    pub r#max_scaled_in_replicas: Box<Option<super::super::types::compute::RegionAutoscalerAutoscalingPolicyScaleInControlMaxScaledInReplicas>>,
    /// How long back autoscaling should look when computing recommendations
    /// to include directives regarding slower scale down, as described above.
    #[builder(into)]
    #[serde(rename = "timeWindowSec")]
    pub r#time_window_sec: Option<i32>,
}
