#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRegionInstanceTemplateScheduling {
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
    pub r#local_ssd_recovery_timeouts: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingLocalSsdRecoveryTimeout>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: PERIODIC
    #[builder(into)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: String,
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "maxRunDurations")]
    pub r#max_run_durations: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingMaxRunDuration>,
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
    pub r#node_affinities: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingNodeAffinity>,
    /// Defines the maintenance behavior for this
    /// instance.
    #[builder(into)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: String,
    /// Defines the behaviour for instances with the instance_termination_action.
    #[builder(into)]
    #[serde(rename = "onInstanceStopActions")]
    pub r#on_instance_stop_actions: Vec<super::super::types::compute::GetRegionInstanceTemplateSchedulingOnInstanceStopAction>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRegionInstanceTemplateScheduling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "automatic_restart".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#automatic_restart,
                )
                .await,
            );
            map.insert(
                "host_error_timeout_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_error_timeout_seconds,
                )
                .await,
            );
            map.insert(
                "instance_termination_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_termination_action,
                )
                .await,
            );
            map.insert(
                "local_ssd_recovery_timeouts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_recovery_timeouts,
                )
                .await,
            );
            map.insert(
                "maintenance_interval".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#maintenance_interval,
                )
                .await,
            );
            map.insert(
                "max_run_durations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_run_durations,
                )
                .await,
            );
            map.insert(
                "min_node_cpus".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_node_cpus,
                )
                .await,
            );
            map.insert(
                "node_affinities".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_affinities,
                )
                .await,
            );
            map.insert(
                "on_host_maintenance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_host_maintenance,
                )
                .await,
            );
            map.insert(
                "on_instance_stop_actions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_instance_stop_actions,
                )
                .await,
            );
            map.insert(
                "preemptible".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#preemptible,
                )
                .await,
            );
            map.insert(
                "provisioning_model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#provisioning_model,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                    r#local_ssd_recovery_timeouts: {
                        let field_value = match fields_map.get("local_ssd_recovery_timeouts") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_ssd_recovery_timeouts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#max_run_durations: {
                        let field_value = match fields_map.get("max_run_durations") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_run_durations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#on_instance_stop_actions: {
                        let field_value = match fields_map.get("on_instance_stop_actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'on_instance_stop_actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
