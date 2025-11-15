#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupInstanceRefreshPreferences {
    /// Alarm Specification for Instance Refresh.
    #[builder(into)]
    #[serde(rename = "alarmSpecification")]
    pub r#alarm_specification: Option<Box<super::super::types::autoscaling::GroupInstanceRefreshPreferencesAlarmSpecification>>,
    /// Automatically rollback if instance refresh fails. Defaults to `false`. This option may only be set to `true` when specifying a `launch_template` or `mixed_instances_policy`.
    #[builder(into)]
    #[serde(rename = "autoRollback")]
    pub r#auto_rollback: Option<bool>,
    /// Number of seconds to wait after a checkpoint. Defaults to `3600`.
    #[builder(into)]
    #[serde(rename = "checkpointDelay")]
    pub r#checkpoint_delay: Option<String>,
    /// List of percentages for each checkpoint. Values must be unique and in ascending order. To replace all instances, the final number must be `100`.
    #[builder(into)]
    #[serde(rename = "checkpointPercentages")]
    pub r#checkpoint_percentages: Option<Vec<i32>>,
    /// Number of seconds until a newly launched instance is configured and ready to use. Default behavior is to use the Auto Scaling Group's health check grace period.
    #[builder(into)]
    #[serde(rename = "instanceWarmup")]
    pub r#instance_warmup: Option<String>,
    /// Amount of capacity in the Auto Scaling group that can be in service and healthy, or pending, to support your workload when an instance refresh is in place, as a percentage of the desired capacity of the Auto Scaling group. Values must be between `100` and `200`, defaults to `100`.
    #[builder(into)]
    #[serde(rename = "maxHealthyPercentage")]
    pub r#max_healthy_percentage: Option<i32>,
    /// Amount of capacity in the Auto Scaling group that must remain healthy during an instance refresh to allow the operation to continue, as a percentage of the desired capacity of the Auto Scaling group. Defaults to `90`.
    #[builder(into)]
    #[serde(rename = "minHealthyPercentage")]
    pub r#min_healthy_percentage: Option<i32>,
    /// Behavior when encountering instances protected from scale in are found. Available behaviors are `Refresh`, `Ignore`, and `Wait`. Default is `Ignore`.
    #[builder(into)]
    #[serde(rename = "scaleInProtectedInstances")]
    pub r#scale_in_protected_instances: Option<String>,
    /// Replace instances that already have your desired configuration. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "skipMatching")]
    pub r#skip_matching: Option<bool>,
    /// Behavior when encountering instances in the `Standby` state in are found. Available behaviors are `Terminate`, `Ignore`, and `Wait`. Default is `Ignore`.
    #[builder(into)]
    #[serde(rename = "standbyInstances")]
    pub r#standby_instances: Option<String>,
}
