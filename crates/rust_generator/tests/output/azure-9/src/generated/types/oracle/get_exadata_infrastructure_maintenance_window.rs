#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExadataInfrastructureMaintenanceWindow {
    /// If true, enables the configuration of a custom action timeout (waiting period) between database servers patching operations.
    #[builder(into)]
    #[serde(rename = "customActionTimeoutEnabled")]
    pub r#custom_action_timeout_enabled: bool,
    #[builder(into)]
    #[serde(rename = "customActionTimeoutInMins")]
    pub r#custom_action_timeout_in_mins: i32,
    /// Days during the week when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "daysOfWeeks")]
    pub r#days_of_weeks: Vec<String>,
    /// The window of hours during the day when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "hoursOfDays")]
    pub r#hours_of_days: Vec<i32>,
    /// Lead time window allows user to set a lead time to prepare for a down time.
    #[builder(into)]
    #[serde(rename = "leadTimeInWeeks")]
    pub r#lead_time_in_weeks: i32,
    /// If true, enables the monthly patching option.
    #[builder(into)]
    #[serde(rename = "monthlyPatchingEnabled")]
    pub r#monthly_patching_enabled: bool,
    /// A `months` block as defined below.
    #[builder(into)]
    #[serde(rename = "months")]
    pub r#months: Vec<String>,
    /// Cloud Exadata Infrastructure node patching method.
    #[builder(into)]
    #[serde(rename = "patchingMode")]
    pub r#patching_mode: String,
    /// The maintenance window scheduling preference.
    #[builder(into)]
    #[serde(rename = "preference")]
    pub r#preference: String,
    /// Weeks during the month when maintenance should be performed.
    #[builder(into)]
    #[serde(rename = "weeksOfMonths")]
    pub r#weeks_of_months: Vec<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetExadataInfrastructureMaintenanceWindow {
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
                    "custom_action_timeout_enabled",
                    &self.r#custom_action_timeout_enabled,
                ),
                to_pulumi_object_field(
                    "custom_action_timeout_in_mins",
                    &self.r#custom_action_timeout_in_mins,
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
                    "lead_time_in_weeks",
                    &self.r#lead_time_in_weeks,
                ),
                to_pulumi_object_field(
                    "monthly_patching_enabled",
                    &self.r#monthly_patching_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetExadataInfrastructureMaintenanceWindow {
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
                    r#custom_action_timeout_enabled: {
                        let field_value = match fields_map.get("custom_action_timeout_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_action_timeout_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_action_timeout_in_mins: {
                        let field_value = match fields_map.get("custom_action_timeout_in_mins") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_action_timeout_in_mins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#lead_time_in_weeks: {
                        let field_value = match fields_map.get("lead_time_in_weeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'lead_time_in_weeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_patching_enabled: {
                        let field_value = match fields_map.get("monthly_patching_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthly_patching_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
