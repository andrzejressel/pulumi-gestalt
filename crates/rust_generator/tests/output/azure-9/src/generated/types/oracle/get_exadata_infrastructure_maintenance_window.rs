#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExadataInfrastructureMaintenanceWindow {
    /// If true, enables the configuration of a custom action timeout (waiting period) between database servers patching operations.
    #[builder(into)]
    pub r#custom_action_timeout_enabled: bool,
    #[builder(into)]
    pub r#custom_action_timeout_in_mins: i32,
    /// Days during the week when maintenance should be performed.
    #[builder(into)]
    pub r#days_of_weeks: Vec<String>,
    /// The window of hours during the day when maintenance should be performed.
    #[builder(into)]
    pub r#hours_of_days: Vec<i32>,
    /// Lead time window allows user to set a lead time to prepare for a down time.
    #[builder(into)]
    pub r#lead_time_in_weeks: i32,
    /// If true, enables the monthly patching option.
    #[builder(into)]
    pub r#monthly_patching_enabled: bool,
    /// A `months` block as defined below.
    #[builder(into)]
    pub r#months: Vec<String>,
    /// Cloud Exadata Infrastructure node patching method.
    #[builder(into)]
    pub r#patching_mode: String,
    /// The maintenance window scheduling preference.
    #[builder(into)]
    pub r#preference: String,
    /// Weeks during the month when maintenance should be performed.
    #[builder(into)]
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
                    "customActionTimeoutEnabled",
                    &self.r#custom_action_timeout_enabled,
                ),
                to_pulumi_object_field(
                    "customActionTimeoutInMins",
                    &self.r#custom_action_timeout_in_mins,
                ),
                to_pulumi_object_field(
                    "daysOfWeeks",
                    &self.r#days_of_weeks,
                ),
                to_pulumi_object_field(
                    "hoursOfDays",
                    &self.r#hours_of_days,
                ),
                to_pulumi_object_field(
                    "leadTimeInWeeks",
                    &self.r#lead_time_in_weeks,
                ),
                to_pulumi_object_field(
                    "monthlyPatchingEnabled",
                    &self.r#monthly_patching_enabled,
                ),
                to_pulumi_object_field(
                    "months",
                    &self.r#months,
                ),
                to_pulumi_object_field(
                    "patchingMode",
                    &self.r#patching_mode,
                ),
                to_pulumi_object_field(
                    "preference",
                    &self.r#preference,
                ),
                to_pulumi_object_field(
                    "weeksOfMonths",
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
                        let field_value = match fields_map.get("customActionTimeoutEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'customActionTimeoutEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_action_timeout_in_mins: {
                        let field_value = match fields_map.get("customActionTimeoutInMins") {
                            Some(value) => value,
                            None => bail!("Missing field 'customActionTimeoutInMins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#days_of_weeks: {
                        let field_value = match fields_map.get("daysOfWeeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'daysOfWeeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hours_of_days: {
                        let field_value = match fields_map.get("hoursOfDays") {
                            Some(value) => value,
                            None => bail!("Missing field 'hoursOfDays' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lead_time_in_weeks: {
                        let field_value = match fields_map.get("leadTimeInWeeks") {
                            Some(value) => value,
                            None => bail!("Missing field 'leadTimeInWeeks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_patching_enabled: {
                        let field_value = match fields_map.get("monthlyPatchingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyPatchingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("patchingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'patchingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("weeksOfMonths") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeksOfMonths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
