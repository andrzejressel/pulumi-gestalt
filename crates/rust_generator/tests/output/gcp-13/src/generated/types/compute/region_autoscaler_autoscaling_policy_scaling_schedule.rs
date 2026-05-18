#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionAutoscalerAutoscalingPolicyScalingSchedule {
    /// An optional description of this resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// A boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// The duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300.
    #[builder(into)]
    #[serde(rename = "durationSec")]
    pub r#duration_sec: i32,
    /// Minimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule.
    #[builder(into)]
    #[serde(rename = "minRequiredReplicas")]
    pub r#min_required_replicas: i32,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field).
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: String,
    /// The time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionAutoscalerAutoscalingPolicyScalingSchedule {
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
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "disabled",
                    &self.r#disabled,
                ),
                to_pulumi_object_field(
                    "duration_sec",
                    &self.r#duration_sec,
                ),
                to_pulumi_object_field(
                    "min_required_replicas",
                    &self.r#min_required_replicas,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "schedule",
                    &self.r#schedule,
                ),
                to_pulumi_object_field(
                    "time_zone",
                    &self.r#time_zone,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionAutoscalerAutoscalingPolicyScalingSchedule {
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
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration_sec: {
                        let field_value = match fields_map.get("duration_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_required_replicas: {
                        let field_value = match fields_map.get("min_required_replicas") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_required_replicas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#schedule: {
                        let field_value = match fields_map.get("schedule") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_zone: {
                        let field_value = match fields_map.get("time_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'time_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
