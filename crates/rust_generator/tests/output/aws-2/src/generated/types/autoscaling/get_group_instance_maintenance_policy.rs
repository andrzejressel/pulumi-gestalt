#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGroupInstanceMaintenancePolicy {
    /// Specifies the upper limit on the number of instances that are in the InService or Pending state with a healthy status during an instance replacement activity.
    #[builder(into)]
    #[serde(rename = "maxHealthyPercentage")]
    pub r#max_healthy_percentage: i32,
    /// Specifies the lower limit on the number of instances that must be in the InService state with a healthy status during an instance replacement activity.
    #[builder(into)]
    #[serde(rename = "minHealthyPercentage")]
    pub r#min_healthy_percentage: i32,
}
