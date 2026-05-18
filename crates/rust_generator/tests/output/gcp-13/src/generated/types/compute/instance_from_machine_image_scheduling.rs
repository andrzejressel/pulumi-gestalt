#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceFromMachineImageScheduling {
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
    pub r#local_ssd_recovery_timeout: Option<Box<super::super::types::compute::InstanceFromMachineImageSchedulingLocalSsdRecoveryTimeout>>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: Option<String>,
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "maxRunDuration")]
    pub r#max_run_duration: Option<Box<super::super::types::compute::InstanceFromMachineImageSchedulingMaxRunDuration>>,
    #[builder(into)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: Option<i32>,
    /// Specifies node affinities or anti-affinities to determine which sole-tenant nodes your instances and managed instance groups will use as host systems.
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Option<Vec<super::super::types::compute::InstanceFromMachineImageSchedulingNodeAffinity>>,
    /// Describes maintenance behavior for the instance. One of MIGRATE or TERMINATE,
    #[builder(into)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: Option<String>,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into)]
    #[serde(rename = "onInstanceStopAction")]
    pub r#on_instance_stop_action: Option<Box<super::super::types::compute::InstanceFromMachineImageSchedulingOnInstanceStopAction>>,
    /// Whether the instance is preemptible.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Option<bool>,
    /// Whether the instance is spot. If this is set as SPOT.
    #[builder(into)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceFromMachineImageScheduling {
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
                    "automatic_restart",
                    &self.r#automatic_restart,
                ),
                to_pulumi_object_field(
                    "host_error_timeout_seconds",
                    &self.r#host_error_timeout_seconds,
                ),
                to_pulumi_object_field(
                    "instance_termination_action",
                    &self.r#instance_termination_action,
                ),
                to_pulumi_object_field(
                    "local_ssd_recovery_timeout",
                    &self.r#local_ssd_recovery_timeout,
                ),
                to_pulumi_object_field(
                    "maintenance_interval",
                    &self.r#maintenance_interval,
                ),
                to_pulumi_object_field(
                    "max_run_duration",
                    &self.r#max_run_duration,
                ),
                to_pulumi_object_field(
                    "min_node_cpus",
                    &self.r#min_node_cpus,
                ),
                to_pulumi_object_field(
                    "node_affinities",
                    &self.r#node_affinities,
                ),
                to_pulumi_object_field(
                    "on_host_maintenance",
                    &self.r#on_host_maintenance,
                ),
                to_pulumi_object_field(
                    "on_instance_stop_action",
                    &self.r#on_instance_stop_action,
                ),
                to_pulumi_object_field(
                    "preemptible",
                    &self.r#preemptible,
                ),
                to_pulumi_object_field(
                    "provisioning_model",
                    &self.r#provisioning_model,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceFromMachineImageScheduling {
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
                        let field_value = match fields_map.get("automatic_restart") {
                            Some(value) => value,
                            None => bail!("Missing field 'automatic_restart' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_error_timeout_seconds: {
                        let field_value = match fields_map.get("host_error_timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_error_timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_termination_action: {
                        let field_value = match fields_map.get("instance_termination_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_termination_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_ssd_recovery_timeout: {
                        let field_value = match fields_map.get("local_ssd_recovery_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_recovery_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#maintenance_interval: {
                        let field_value = match fields_map.get("maintenance_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'maintenance_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_run_duration: {
                        let field_value = match fields_map.get("max_run_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_run_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_node_cpus: {
                        let field_value = match fields_map.get("min_node_cpus") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_node_cpus' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#node_affinities: {
                        let field_value = match fields_map.get("node_affinities") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_affinities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_host_maintenance: {
                        let field_value = match fields_map.get("on_host_maintenance") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_host_maintenance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#on_instance_stop_action: {
                        let field_value = match fields_map.get("on_instance_stop_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_instance_stop_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("provisioning_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'provisioning_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
