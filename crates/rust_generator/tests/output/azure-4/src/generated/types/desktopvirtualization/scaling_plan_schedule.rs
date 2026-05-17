#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScalingPlanSchedule {
    /// A list of Days of the Week on which this schedule will be used. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, and `Sunday`
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// The name of the schedule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The load Balancing Algorithm to use during Off-Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "offPeakLoadBalancingAlgorithm")]
    pub r#off_peak_load_balancing_algorithm: String,
    /// The time at which Off-Peak scaling will begin. This is also the end-time for the Ramp-Down period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "offPeakStartTime")]
    pub r#off_peak_start_time: String,
    /// The load Balancing Algorithm to use during Peak Hours. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "peakLoadBalancingAlgorithm")]
    pub r#peak_load_balancing_algorithm: String,
    /// The time at which Peak scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "peakStartTime")]
    pub r#peak_start_time: String,
    /// This is the value in percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-down and off-peak hours. For example, if capacity threshold is specified as 60% and your total host pool capacity is 100 sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of 60 sessions.
    #[builder(into)]
    #[serde(rename = "rampDownCapacityThresholdPercent")]
    pub r#ramp_down_capacity_threshold_percent: i32,
    /// Whether users will be forced to log-off session hosts once the `ramp_down_wait_time_minutes` value has been exceeded during the Ramp-Down period. Possible values are `true` and `false`.
    #[builder(into)]
    #[serde(rename = "rampDownForceLogoffUsers")]
    pub r#ramp_down_force_logoff_users: bool,
    /// The load Balancing Algorithm to use during the Ramp-Down period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "rampDownLoadBalancingAlgorithm")]
    pub r#ramp_down_load_balancing_algorithm: String,
    /// The minimum percentage of session host virtual machines that you would like to get to for ramp-down and off-peak hours. For example, if Minimum percentage of hosts is specified as 10% and total number of session hosts in your host pool is 10, autoscale will ensure a minimum of 1 session host is available to take user connections.
    #[builder(into)]
    #[serde(rename = "rampDownMinimumHostsPercent")]
    pub r#ramp_down_minimum_hosts_percent: i32,
    /// The notification message to send to users during Ramp-Down period when they are required to log-off.
    #[builder(into)]
    #[serde(rename = "rampDownNotificationMessage")]
    pub r#ramp_down_notification_message: String,
    /// The time at which Ramp-Down scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "rampDownStartTime")]
    pub r#ramp_down_start_time: String,
    /// Controls Session Host shutdown behaviour during Ramp-Down period. Session Hosts can either be shutdown when all sessions on the Session Host have ended, or when there are no Active sessions left on the Session Host. Possible values are `ZeroSessions` and `ZeroActiveSessions`.
    #[builder(into)]
    #[serde(rename = "rampDownStopHostsWhen")]
    pub r#ramp_down_stop_hosts_when: String,
    /// The number of minutes during Ramp-Down period that autoscale will wait after setting the session host VMs to drain mode, notifying any currently signed in users to save their work before forcing the users to logoff. Once all user sessions on the session host VM have been logged off, Autoscale will shut down the VM.
    #[builder(into)]
    #[serde(rename = "rampDownWaitTimeMinutes")]
    pub r#ramp_down_wait_time_minutes: i32,
    /// This is the value of percentage of used host pool capacity that will be considered to evaluate whether to turn on/off virtual machines during the ramp-up and peak hours. For example, if capacity threshold is specified as `60%` and your total host pool capacity is `100` sessions, autoscale will turn on additional session hosts once the host pool exceeds a load of `60` sessions.
    #[builder(into)]
    #[serde(rename = "rampUpCapacityThresholdPercent")]
    pub r#ramp_up_capacity_threshold_percent: Option<i32>,
    /// The load Balancing Algorithm to use during the Ramp-Up period. Possible values are `DepthFirst` and `BreadthFirst`.
    #[builder(into)]
    #[serde(rename = "rampUpLoadBalancingAlgorithm")]
    pub r#ramp_up_load_balancing_algorithm: String,
    /// Specifies the minimum percentage of session host virtual machines to start during ramp-up for peak hours. For example, if Minimum percentage of hosts is specified as `10%` and total number of session hosts in your host pool is `10`, autoscale will ensure a minimum of `1` session host is available to take user connections.
    #[builder(into)]
    #[serde(rename = "rampUpMinimumHostsPercent")]
    pub r#ramp_up_minimum_hosts_percent: Option<i32>,
    /// The time at which Ramp-Up scaling will begin. This is also the end-time for the Ramp-Up period. The time must be specified in "HH:MM" format.
    #[builder(into)]
    #[serde(rename = "rampUpStartTime")]
    pub r#ramp_up_start_time: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScalingPlanSchedule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "days_of_weeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "off_peak_load_balancing_algorithm",
                    &self.r#off_peak_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "off_peak_start_time",
                    &self.r#off_peak_start_time,
                ),
                to_pulumi_object_field(
                    "peak_load_balancing_algorithm",
                    &self.r#peak_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "peak_start_time",
                    &self.r#peak_start_time,
                ),
                to_pulumi_object_field(
                    "ramp_down_capacity_threshold_percent",
                    &self.r#ramp_down_capacity_threshold_percent,
                ),
                to_pulumi_object_field(
                    "ramp_down_force_logoff_users",
                    &self.r#ramp_down_force_logoff_users,
                ),
                to_pulumi_object_field(
                    "ramp_down_load_balancing_algorithm",
                    &self.r#ramp_down_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "ramp_down_minimum_hosts_percent",
                    &self.r#ramp_down_minimum_hosts_percent,
                ),
                to_pulumi_object_field(
                    "ramp_down_notification_message",
                    &self.r#ramp_down_notification_message,
                ),
                to_pulumi_object_field(
                    "ramp_down_start_time",
                    &self.r#ramp_down_start_time,
                ),
                to_pulumi_object_field(
                    "ramp_down_stop_hosts_when",
                    &self.r#ramp_down_stop_hosts_when,
                ),
                to_pulumi_object_field(
                    "ramp_down_wait_time_minutes",
                    &self.r#ramp_down_wait_time_minutes,
                ),
                to_pulumi_object_field(
                    "ramp_up_capacity_threshold_percent",
                    &self.r#ramp_up_capacity_threshold_percent,
                ),
                to_pulumi_object_field(
                    "ramp_up_load_balancing_algorithm",
                    &self.r#ramp_up_load_balancing_algorithm,
                ),
                to_pulumi_object_field(
                    "ramp_up_minimum_hosts_percent",
                    &self.r#ramp_up_minimum_hosts_percent,
                ),
                to_pulumi_object_field(
                    "ramp_up_start_time",
                    &self.r#ramp_up_start_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("days_of_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("off_peak_load_balancing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'off_peak_load_balancing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#off_peak_start_time: {
                        let field_value = match fields_map.get("off_peak_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'off_peak_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peak_load_balancing_algorithm: {
                        let field_value = match fields_map.get("peak_load_balancing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'peak_load_balancing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peak_start_time: {
                        let field_value = match fields_map.get("peak_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'peak_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_capacity_threshold_percent: {
                        let field_value = match fields_map.get("ramp_down_capacity_threshold_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_capacity_threshold_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_force_logoff_users: {
                        let field_value = match fields_map.get("ramp_down_force_logoff_users") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_force_logoff_users' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_load_balancing_algorithm: {
                        let field_value = match fields_map.get("ramp_down_load_balancing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_load_balancing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_minimum_hosts_percent: {
                        let field_value = match fields_map.get("ramp_down_minimum_hosts_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_minimum_hosts_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_notification_message: {
                        let field_value = match fields_map.get("ramp_down_notification_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_notification_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_start_time: {
                        let field_value = match fields_map.get("ramp_down_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_stop_hosts_when: {
                        let field_value = match fields_map.get("ramp_down_stop_hosts_when") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_stop_hosts_when' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_down_wait_time_minutes: {
                        let field_value = match fields_map.get("ramp_down_wait_time_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_down_wait_time_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_capacity_threshold_percent: {
                        let field_value = match fields_map.get("ramp_up_capacity_threshold_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_up_capacity_threshold_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_load_balancing_algorithm: {
                        let field_value = match fields_map.get("ramp_up_load_balancing_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_up_load_balancing_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_minimum_hosts_percent: {
                        let field_value = match fields_map.get("ramp_up_minimum_hosts_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_up_minimum_hosts_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ramp_up_start_time: {
                        let field_value = match fields_map.get("ramp_up_start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'ramp_up_start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
