#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceScheduling {
    /// Specifies if the instance should be
    /// restarted if it was terminated by Compute Engine (not a user).
    /// Defaults to true.
    #[builder(into)]
    #[serde(rename = "automaticRestart")]
    pub r#automatic_restart: Option<bool>,
    /// Specifies the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used.
    #[builder(into)]
    #[serde(rename = "hostErrorTimeoutSeconds")]
    pub r#host_error_timeout_seconds: Option<i32>,
    /// Describe the type of termination action for VM. Can be `STOP` or `DELETE`.  Read more on [here](https://cloud.google.com/compute/docs/instances/create-use-spot)
    #[builder(into)]
    #[serde(rename = "instanceTerminationAction")]
    pub r#instance_termination_action: Option<String>,
    /// Specifies the maximum amount of time a Local Ssd Vm should wait while
    ///   recovery of the Local Ssd state is attempted. Its value should be in
    ///   between 0 and 168 hours with hour granularity and the default value being 1
    ///   hour.
    #[builder(into)]
    #[serde(rename = "localSsdRecoveryTimeout")]
    pub r#local_ssd_recovery_timeout: Option<Box<super::super::types::compute::InstanceSchedulingLocalSsdRecoveryTimeout>>,
    /// Specifies the frequency of planned maintenance events. The accepted values are: `PERIODIC`.
    #[builder(into)]
    #[serde(rename = "maintenanceInterval")]
    pub r#maintenance_interval: Option<String>,
    /// The duration of the instance. Instance will run and be terminated after then, the termination action could be defined in `instance_termination_action`. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "maxRunDuration")]
    pub r#max_run_duration: Option<Box<super::super::types::compute::InstanceSchedulingMaxRunDuration>>,
    /// The minimum number of virtual CPUs this instance will consume when running on a sole-tenant node.
    #[builder(into)]
    #[serde(rename = "minNodeCpus")]
    pub r#min_node_cpus: Option<i32>,
    /// Specifies node affinities or anti-affinities
    /// to determine which sole-tenant nodes your instances and managed instance
    /// groups will use as host systems. Read more on sole-tenant node creation
    /// [here](https://cloud.google.com/compute/docs/nodes/create-nodes).
    /// Structure documented below.
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Option<Vec<super::super::types::compute::InstanceSchedulingNodeAffinity>>,
    /// Describes maintenance behavior for the
    /// instance. Can be MIGRATE or TERMINATE, for more info, read
    /// [here](https://cloud.google.com/compute/docs/instances/setting-instance-scheduling-options).
    #[builder(into)]
    #[serde(rename = "onHostMaintenance")]
    pub r#on_host_maintenance: Option<String>,
    /// Specifies the action to be performed when the instance is terminated using `max_run_duration` and `STOP` `instance_termination_action`. Only support `true` `discard_local_ssd` at this point. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "onInstanceStopAction")]
    pub r#on_instance_stop_action: Option<Box<super::super::types::compute::InstanceSchedulingOnInstanceStopAction>>,
    /// Specifies if the instance is preemptible.
    /// If this field is set to true, then `automatic_restart` must be
    /// set to false.  Defaults to false.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Option<bool>,
    /// Describe the type of preemptible VM. This field accepts the value `STANDARD` or `SPOT`. If the value is `STANDARD`, there will be no discount. If this   is set to `SPOT`,
    /// `preemptible` should be `true` and `automatic_restart` should be
    /// `false`. For more info about
    /// `SPOT`, read [here](https://cloud.google.com/compute/docs/instances/spot)
    #[builder(into)]
    #[serde(rename = "provisioningModel")]
    pub r#provisioning_model: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceScheduling {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
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
                "local_ssd_recovery_timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_ssd_recovery_timeout,
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
                "max_run_duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_run_duration,
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
                "on_instance_stop_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#on_instance_stop_action,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceScheduling {
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
