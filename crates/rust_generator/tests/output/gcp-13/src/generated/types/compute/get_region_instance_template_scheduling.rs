#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRegionInstanceTemplateScheduling {
    /// Specifies whether the instance should be
    /// automatically restarted if it is terminated by Compute Engine (not
    /// terminated by a user). This defaults to true.
    #[builder(into)]
    pub r#automatic_restart: bool,
    /// Beta Time in seconds for host error detection.
    #[builder(into)]
    pub r#host_error_timeout_seconds: i32,
    /// Describe the type of termination action for `SPOT` VM. Can be `STOP` or `DELETE`.  Read more on [here](https://cloud.google.com/compute/docs/instances/create-use-spot)
    #[builder(into)]
    pub r#instance_termination_action: String,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into)]
    pub r#local_ssd_recovery_timeouts: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingLocalSsdRecoveryTimeout>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into)]
    pub r#maintenance_interval: String,
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    pub r#max_run_durations: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingMaxRunDuration>,
    /// Minimum number of cpus for the instance.
    #[builder(into)]
    pub r#min_node_cpus: i32,
    /// Specifies node affinities or anti-affinities
    /// to determine which sole-tenant nodes your instances and managed instance
    /// groups will use as host systems. Read more on sole-tenant node creation
    /// [here](https://cloud.google.com/compute/docs/nodes/create-nodes).
    /// Structure documented below.
    #[builder(into)]
    pub r#node_affinities: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingNodeAffinity>,
    /// Defines the maintenance behavior for this
    /// instance.
    #[builder(into)]
    pub r#on_host_maintenance: String,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into)]
    pub r#on_instance_stop_actions: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingOnInstanceStopAction>,
    /// Allows instance to be preempted. This defaults to
    /// false. Read more on this
    /// [here](https://cloud.google.com/compute/docs/instances/preemptible).
    #[builder(into)]
    pub r#preemptible: bool,
    /// Describe the type of preemptible VM.
    #[builder(into)]
    pub r#provisioning_model: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRegionInstanceTemplateScheduling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "automaticRestart",
                    &self.r#automatic_restart,
                ),
                to_pulumi_object_field(
                    "hostErrorTimeoutSeconds",
                    &self.r#host_error_timeout_seconds,
                ),
                to_pulumi_object_field(
                    "instanceTerminationAction",
                    &self.r#instance_termination_action,
                ),
                to_pulumi_object_field(
                    "localSsdRecoveryTimeouts",
                    &self.r#local_ssd_recovery_timeouts,
                ),
                to_pulumi_object_field(
                    "maintenanceInterval",
                    &self.r#maintenance_interval,
                ),
                to_pulumi_object_field(
                    "maxRunDurations",
                    &self.r#max_run_durations,
                ),
                to_pulumi_object_field(
                    "minNodeCpus",
                    &self.r#min_node_cpus,
                ),
                to_pulumi_object_field(
                    "nodeAffinities",
                    &self.r#node_affinities,
                ),
                to_pulumi_object_field(
                    "onHostMaintenance",
                    &self.r#on_host_maintenance,
                ),
                to_pulumi_object_field(
                    "onInstanceStopActions",
                    &self.r#on_instance_stop_actions,
                ),
                to_pulumi_object_field(
                    "preemptible",
                    &self.r#preemptible,
                ),
                to_pulumi_object_field(
                    "provisioningModel",
                    &self.r#provisioning_model,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRegionInstanceTemplateScheduling {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#automatic_restart: {
                        let field_value = match fields_map.get("automaticRestart") {
                            Some(value) => value,
                            None => bail!("Missing field 'automaticRestart' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_error_timeout_seconds: {
                        let field_value = match fields_map.get("hostErrorTimeoutSeconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostErrorTimeoutSeconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_termination_action: {
                        let field_value = match fields_map.get("instanceTerminationAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'instanceTerminationAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_recovery_timeouts: {
                        let field_value = match fields_map.get("localSsdRecoveryTimeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'localSsdRecoveryTimeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_interval: {
                        let field_value = match fields_map.get("maintenanceInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenanceInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_run_durations: {
                        let field_value = match fields_map.get("maxRunDurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRunDurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_node_cpus: {
                        let field_value = match fields_map.get("minNodeCpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'minNodeCpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_affinities: {
                        let field_value = match fields_map.get("nodeAffinities") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeAffinities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_host_maintenance: {
                        let field_value = match fields_map.get("onHostMaintenance") {
                            Some(value) => value,
                            None => bail!("Missing field 'onHostMaintenance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_instance_stop_actions: {
                        let field_value = match fields_map.get("onInstanceStopActions") {
                            Some(value) => value,
                            None => bail!("Missing field 'onInstanceStopActions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preemptible: {
                        let field_value = match fields_map.get("preemptible") {
                            Some(value) => value,
                            None => bail!("Missing field 'preemptible' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#provisioning_model: {
                        let field_value = match fields_map.get("provisioningModel") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioningModel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
