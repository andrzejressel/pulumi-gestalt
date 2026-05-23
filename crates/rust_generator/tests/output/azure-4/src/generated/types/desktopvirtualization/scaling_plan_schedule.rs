#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScalingPlanSchedule {
    /// A list of Days of the Week on which this schedule will be used. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`
    #[builder(into)]
    pub r#days_of_weeks: Vec<String>,
    /// The name of the schedule.
    #[builder(into)]
    pub r#name: String,
    /// The load Balancing Algorithm to use during Off-Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    pub r#off_peak_load_balancing_algorithm: String,
    /// The time at which Off-Peak scaling will begin. This is also the end-time for the Ramp-Down period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    pub r#off_peak_start_time: String,
    /// The load Balancing Algorithm to use during Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    pub r#peak_load_balancing_algorithm: String,
    /// The time at which Peak scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    pub r#peak_start_time: String,
    /// This is the value in percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-down and off-peak hours. For example, if capacity threshold is specified as 60% and your total host pool capacity is 100 sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of 60 sessions.
    #[builder(into)]
    pub r#ramp_down_capacity_threshold_percent: i32,
    /// Whether users will be forced to log-off session hosts once the `ramp_down_wait_time_minutes` value has been exceeded during the Ramp-Down period. Possible values are `true` and `false`.
    #[builder(into)]
    pub r#ramp_down_force_logoff_users: bool,
    /// The load Balancing Algorithm to use during the Ramp-Down period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    pub r#ramp_down_load_balancing_algorithm: String,
    /// The minimum percentage of session host virtual machines that you would like to get to for ramp-down and off-peak hours. For example, if Minimum percentage of hosts is specified as 10% and total number of session hosts in your host pool is 10, autoscale will ensure a minimum of 1 session host is available to take user connections.
    #[builder(into)]
    pub r#ramp_down_minimum_hosts_percent: i32,
    /// The notification message to send to users during Ramp-Down period when they are required to log-off.
    #[builder(into)]
    pub r#ramp_down_notification_message: String,
    /// The time at which Ramp-Down scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    pub r#ramp_down_start_time: String,
    /// Controls Session Host shutdown behaviour during Ramp-Down period. Session Hosts can either be shutdown when all sessions on the Session Host have ended, or when there are no Active sessions left on the Session Host. Possible values are `ZeroSessions` and `ZeroActiveSessions`.
    #[builder(into)]
    pub r#ramp_down_stop_hosts_when: String,
    /// The number of minutes during Ramp-Down period that autoscale will wait after setting the session host VMs to drain mode, notifying any currently signed in users to save their work before forcing the users to logoff. Once all user sessions on the session host VM have been logged off, Autoscale will shut down the VM.
    #[builder(into)]
    pub r#ramp_down_wait_time_minutes: i32,
    /// This is the value of percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-up and peak hours. For example, if capacity threshold is specified as `60%` and your total host pool capacity is `100` sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of `60` sessions.
    #[builder(into)]
    pub r#ramp_up_capacity_threshold_percent: Option<i32>,
    /// The load Balancing Algorithm to use during the Ramp-Up period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    pub r#ramp_up_load_balancing_algorithm: String,
    /// Specifies the minimum percentage of session host virtual machines to start during ramp-up for peak hours. For example, if Minimum percentage of hosts is specified as `10%` and total number of session hosts in your host pool is `10`, autoscale will ensure a minimum of `1` session host is available to take user connections.
    #[builder(into)]
    pub r#ramp_up_minimum_hosts_percent: Option<i32>,
    /// The time at which Ramp-Up scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    pub r#ramp_up_start_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScalingPlanSchedule {
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
                    "daysOfWeeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "offPeakLoadBalancingAlgorithm",
                    &self.r#off_peak_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "offPeakStartTime",
                    &self.r#off_peak_start_time,
                ),
                to_pulumi_object_field(
                    "peakLoadBalancingAlgorithm",
                    &self.r#peak_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "peakStartTime",
                    &self.r#peak_start_time,
                ),
                to_pulumi_object_field(
                    "rampDownCapacityThresholdPercent",
                    &self.r#ramp_down_capacity_threshold_percent,
                ),
                to_pulumi_object_field(
                    "rampDownForceLogoffUsers",
                    &self.r#ramp_down_force_logoff_users,
                ),
                to_pulumi_object_field(
                    "rampDownLoadBalancingAlgorithm",
                    &self.r#ramp_down_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "rampDownMinimumHostsPercent",
                    &self.r#ramp_down_minimum_hosts_percent,
                ),
                to_pulumi_object_field(
                    "rampDownNotificationMessage",
                    &self.r#ramp_down_notification_message,
                ),
                to_pulumi_object_field(
                    "rampDownStartTime",
                    &self.r#ramp_down_start_time,
                ),
                to_pulumi_object_field(
                    "rampDownStopHostsWhen",
                    &self.r#ramp_down_stop_hosts_when,
                ),
                to_pulumi_object_field(
                    "rampDownWaitTimeMinutes",
                    &self.r#ramp_down_wait_time_minutes,
                ),
                to_pulumi_object_field(
                    "rampUpCapacityThresholdPercent",
                    &self.r#ramp_up_capacity_threshold_percent,
                ),
                to_pulumi_object_field(
                    "rampUpLoadBalancingAlgorithm",
                    &self.r#ramp_up_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "rampUpMinimumHostsPercent",
                    &self.r#ramp_up_minimum_hosts_percent,
                ),
                to_pulumi_object_field(
                    "rampUpStartTime",
                    &self.r#ramp_up_start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScalingPlanSchedule {
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
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("daysOfWeeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysOfWeeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#off_peak_load_balancing_algorithm: {
                        let field_value = match fields_map.get("offPeakLoadBalancingAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'offPeakLoadBalancingAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#off_peak_start_time: {
                        let field_value = match fields_map.get("offPeakStartTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'offPeakStartTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peak_load_balancing_algorithm: {
                        let field_value = match fields_map.get("peakLoadBalancingAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'peakLoadBalancingAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peak_start_time: {
                        let field_value = match fields_map.get("peakStartTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'peakStartTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_capacity_threshold_percent: {
                        let field_value = match fields_map.get("rampDownCapacityThresholdPercent") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownCapacityThresholdPercent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_force_logoff_users: {
                        let field_value = match fields_map.get("rampDownForceLogoffUsers") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownForceLogoffUsers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_load_balancing_algorithm: {
                        let field_value = match fields_map.get("rampDownLoadBalancingAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownLoadBalancingAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_minimum_hosts_percent: {
                        let field_value = match fields_map.get("rampDownMinimumHostsPercent") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownMinimumHostsPercent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_notification_message: {
                        let field_value = match fields_map.get("rampDownNotificationMessage") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownNotificationMessage' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_start_time: {
                        let field_value = match fields_map.get("rampDownStartTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownStartTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_stop_hosts_when: {
                        let field_value = match fields_map.get("rampDownStopHostsWhen") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownStopHostsWhen' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_wait_time_minutes: {
                        let field_value = match fields_map.get("rampDownWaitTimeMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampDownWaitTimeMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_capacity_threshold_percent: {
                        let field_value = match fields_map.get("rampUpCapacityThresholdPercent") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampUpCapacityThresholdPercent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_load_balancing_algorithm: {
                        let field_value = match fields_map.get("rampUpLoadBalancingAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampUpLoadBalancingAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_minimum_hosts_percent: {
                        let field_value = match fields_map.get("rampUpMinimumHostsPercent") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampUpMinimumHostsPercent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_start_time: {
                        let field_value = match fields_map.get("rampUpStartTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'rampUpStartTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
