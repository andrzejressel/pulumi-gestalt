#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceFromTemplateScheduling {
    /// Specifies if the instance should be restarted if it was terminated by Compute Engine (not a user).
    #[builder(into)]
    #[serde(rename = "automaticRestart")]
    pub r#automatic_restart: Option<bool>,
    /// Specify the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used.
    #[builder(into)]
    #[serde(rename = "hostErrorTimeoutSeconds")]
    pub r#host_error_timeout_seconds: Option<i32>,
    /// Specifies the action GCE should take when SPOT VM is preempted.
    #[builder(into)]
    #[serde(rename = "instanceTerminationAction")]
    pub r#instance_termination_action: Option<String>,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into)]
    #[serde(rename = "localSsdRecoveryTimeout")]
    pub r#local_ssd_recovery_timeout: Option<Box<super::super::types::compute::InstanceFromTemplateSchedulingLocalSsdRecoveryTimeout>>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: Option<String>,
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "maxRunDuration")]
    pub r#max_run_duration: Option<Box<super::super::types::compute::InstanceFromTemplateSchedulingMaxRunDuration>>,
    #[builder(into)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: Option<i32>,
    /// Specifies node affinities or anti-affinities to determine which sole-tenant nodes your instances and managed instance groups will use as host systems.
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Option<Vec<super::super::types::compute::InstanceFromTemplateSchedulingNodeAffinity>>,
    /// Describes maintenance behavior for the instance. One of MIGRATE or TERMINATE,
    #[builder(into)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: Option<String>,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into)]
    #[serde(rename = "onInstanceStopAction")]
    pub r#on_instance_stop_action: Option<Box<super::super::types::compute::InstanceFromTemplateSchedulingOnInstanceStopAction>>,
    /// Whether the instance is preemptible.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Option<bool>,
    /// Whether the instance is spot. If this is set as SPOT.
    #[builder(into)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: Option<String>,
}
