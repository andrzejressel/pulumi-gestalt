#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterMaintenanceWindowNodeOs {
    /// The day of the month for the maintenance run. Required in combination with AbsoluteMonthly frequency. Value between 0 and 31 (inclusive).
    #[builder(into)]
    #[serde(rename = "dayOfMonth")]
    pub r#day_of_month: Option<i32>,
    /// The day of the week for the maintenance run. Required in combination with weekly frequency. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Option<String>,
    /// The duration of the window for maintenance to run in hours. Possible options are between `4` to `24`.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: i32,
    /// Frequency of maintenance. Possible options are `Daily`, `Weekly`, `AbsoluteMonthly` and `RelativeMonthly`.
    #[builder(into)]
    #[serde(rename = "frequency")]
    pub r#frequency: String,
    /// The interval for maintenance runs. Depending on the frequency this interval is week or month based.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: i32,
    /// One or more `not_allowed` block as defined below.
    #[builder(into)]
    #[serde(rename = "notAlloweds")]
    pub r#not_alloweds: Option<Vec<super::super::types::containerservice::KubernetesClusterMaintenanceWindowNodeOsNotAllowed>>,
    /// The date on which the maintenance window begins to take effect.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Option<String>,
    /// The time for maintenance to begin, based on the timezone determined by `utc_offset`. Format is `HH:mm`.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Used to determine the timezone for cluster maintenance.
    #[builder(into)]
    #[serde(rename = "utcOffset")]
    pub r#utc_offset: Option<String>,
    /// The week in the month used for the maintenance run. Options are `First`, `Second`, `Third`, `Fourth`, and `Last`.
    #[builder(into)]
    #[serde(rename = "weekIndex")]
    pub r#week_index: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterMaintenanceWindowNodeOs {
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
                    "day_of_month",
                    &self.r#day_of_month,
                ),
                to_pulumi_object_field(
                    "day_of_week",
                    &self.r#day_of_week,
                ),
                to_pulumi_object_field(
                    "duration",
                    &self.r#duration,
                ),
                to_pulumi_object_field(
                    "frequency",
                    &self.r#frequency,
                ),
                to_pulumi_object_field(
                    "interval",
                    &self.r#interval,
                ),
                to_pulumi_object_field(
                    "not_alloweds",
                    &self.r#not_alloweds,
                ),
                to_pulumi_object_field(
                    "start_date",
                    &self.r#start_date,
                ),
                to_pulumi_object_field(
                    "start_time",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "utc_offset",
                    &self.r#utc_offset,
                ),
                to_pulumi_object_field(
                    "week_index",
                    &self.r#week_index,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterMaintenanceWindowNodeOs {
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
                    r#day_of_month: {
                        let field_value = match fields_map.get("day_of_month") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_month' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#day_of_week: {
                        let field_value = match fields_map.get("day_of_week") {
                            Some(value) => value,
                            None => bail!("Missing field 'day_of_week' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#frequency: {
                        let field_value = match fields_map.get("frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#interval: {
                        let field_value = match fields_map.get("interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#not_alloweds: {
                        let field_value = match fields_map.get("not_alloweds") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_alloweds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_date: {
                        let field_value = match fields_map.get("start_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#utc_offset: {
                        let field_value = match fields_map.get("utc_offset") {
                            Some(value) => value,
                            None => bail!("Missing field 'utc_offset' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#week_index: {
                        let field_value = match fields_map.get("week_index") {
                            Some(value) => value,
                            None => bail!("Missing field 'week_index' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
