#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceTemplateScheduling {
    /// Specifies whether the instance should be
    /// automatically restarted if it is terminated by Compute Engine (not
    /// terminated by a user). This defaults to true.
    #[builder(into)]
    #[serde(rename = "automaticRestart")]
    pub r#automatic_restart: bool,
    /// Beta Time in seconds for host error detection.
    #[builder(into)]
    #[serde(rename = "hostErrorTimeoutSeconds")]
    pub r#host_error_timeout_seconds: i32,
    /// Describe the type of termination action for `SPOT` VM. Can be `STOP` or `DELETE`.  Read more on [here](https://cloud.google.com/compute/docs/instances/create-use-spot)
    #[builder(into)]
    #[serde(rename = "instanceTerminationAction")]
    pub r#instance_termination_action: String,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into)]
    #[serde(rename = "localSsdRecoveryTimeouts")]
    pub r#local_ssd_recovery_timeouts: Vec<super::super::types::compute::GetInstanceTemplateSchedulingLocalSsdRecoveryTimeout>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: String,
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "maxRunDurations")]
    pub r#max_run_durations: Vec<super::super::types::compute::GetInstanceTemplateSchedulingMaxRunDuration>,
    /// Minimum number of cpus for the instance.
    #[builder(into)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: i32,
    /// Specifies node affinities or anti-affinities
    /// to determine which sole-tenant nodes your instances and managed instance
    /// groups will use as host systems. Read more on sole-tenant node creation
    /// [here](https://cloud.google.com/compute/docs/nodes/create-nodes).
    /// Structure documented below.
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Vec<super::super::types::compute::GetInstanceTemplateSchedulingNodeAffinity>,
    /// Defines the maintenance behavior for this
    /// instance.
    #[builder(into)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: String,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into)]
    #[serde(rename = "onInstanceStopActions")]
    pub r#on_instance_stop_actions: Vec<super::super::types::compute::GetInstanceTemplateSchedulingOnInstanceStopAction>,
    /// Allows instance to be preempted. This defaults to
    /// false. Read more on this
    /// [here](https://cloud.google.com/compute/docs/instances/preemptible).
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: bool,
    /// Describe the type of preemptible VM.
    #[builder(into)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: String,
}
