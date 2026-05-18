#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow {
    /// Determines the amount of time the system will wait before the start of each
    /// database server patching operation. Custom action timeout is in minutes and
    /// valid value is between 15 to 120 (inclusive).
    #[builder(into)]
    #[serde(rename = "customActionTimeoutMins")]
    pub r#custom_action_timeout_mins: i32,
    /// Days during the week when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// The window of hours during the day when maintenance should be performed.
    /// The window is a 4 hour slot. Valid values are:
    ///   0 - represents time slot 0:00 - 3:59 UTC
    ///   4 - represents time slot 4:00 - 7:59 UTC
    ///   8 - represents time slot 8:00 - 11:59 UTC
    ///   12 - represents time slot 12:00 - 15:59 UTC
    ///   16 - represents time slot 16:00 - 19:59 UTC
    ///   20 - represents time slot 20:00 - 23:59 UTC
    #[builder(into)]
    #[serde(rename = "hoursOfDays")]
    pub r#hours_of_days: Vec<i32>,
    /// If true, enables the configuration of a custom action timeout (waiting
    /// period) between database server patching operations.
    #[builder(into)]
    #[serde(rename = "isCustomActionTimeoutEnabled")]
    pub r#is_custom_action_timeout_enabled: bool,
    /// Lead time window allows user to set a lead time to prepare for a down time.
    /// The lead time is in weeks and valid value is between 1 to 4.
    #[builder(into)]
    #[serde(rename = "leadTimeWeek")]
    pub r#lead_time_week: i32,
    /// Months during the year when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Vec<String>,
    /// Cloud CloudExadataInfrastructure node patching method, either "ROLLING"
    ///  or "NONROLLING". Default value is ROLLING. 
    ///  Possible values:
    ///  PATCHING_MODE_UNSPECIFIED
    /// ROLLING
    /// NON_ROLLING
    #[builder(into)]
    #[serde(rename = "patchingMode")]
    pub r#patching_mode: String,
    /// The maintenance window scheduling preference. 
    ///  Possible values:
    ///  MAINTENANCE_WINDOW_PREFERENCE_UNSPECIFIED
    /// CUSTOM_PREFERENCE
    /// NO_PREFERENCE
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: String,
    /// Weeks during the month when maintenance should be performed. Weeks start on
    /// the 1st, 8th, 15th, and 22nd days of the month, and have a duration of 7
    /// days. Weeks start and end based on calendar dates, not days of the week.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Vec<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow {
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
                    "custom_action_timeout_mins",
                    &self.r#custom_action_timeout_mins,
                ),
                to_pulumi_object_field(
                    "days_of_weeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "hours_of_days",
                    &self.r#hours_of_days,
                ),
                to_pulumi_object_field(
                    "is_custom_action_timeout_enabled",
                    &self.r#is_custom_action_timeout_enabled,
                ),
                to_pulumi_object_field(
                    "lead_time_week",
                    &self.r#lead_time_week,
                ),
                to_pulumi_object_field(
                    "months",
                    &self.r#months,
                ),
                to_pulumi_object_field(
                    "patching_mode",
                    &self.r#patching_mode,
                ),
                to_pulumi_object_field(
                    "preference",
                    &self.r#preference,
                ),
                to_pulumi_object_field(
                    "weeks_of_months",
                    &self.r#weeks_of_months,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCloudExadataInfrastructuresCloudExadataInfrastructurePropertyMaintenanceWindow {
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
                    r#custom_action_timeout_mins: {
                        let field_value = match fields_map.get("custom_action_timeout_mins") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_action_timeout_mins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("days_of_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'days_of_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hours_of_days: {
                        let field_value = match fields_map.get("hours_of_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'hours_of_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_custom_action_timeout_enabled: {
                        let field_value = match fields_map.get("is_custom_action_timeout_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_custom_action_timeout_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lead_time_week: {
                        let field_value = match fields_map.get("lead_time_week") {
                            Some(value) => value,
                            None => bail!("Missing field 'lead_time_week' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#months: {
                        let field_value = match fields_map.get("months") {
                            Some(value) => value,
                            None => bail!("Missing field 'months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patching_mode: {
                        let field_value = match fields_map.get("patching_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'patching_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preference: {
                        let field_value = match fields_map.get("preference") {
                            Some(value) => value,
                            None => bail!("Missing field 'preference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weeks_of_months: {
                        let field_value = match fields_map.get("weeks_of_months") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeks_of_months' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
