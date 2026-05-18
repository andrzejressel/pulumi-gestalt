#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskTriggerSpec {
    /// Prevent the task from executing. This does not cancel already running tasks. It is intended to temporarily disable RECURRING tasks.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Number of retry attempts before aborting. Set to zero to never attempt to retry a failed task.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Option<i32>,
    /// Cron schedule (https://en.wikipedia.org/wiki/Cron) for running tasks periodically. To explicitly set a timezone to the cron tab, apply a prefix in the cron tab: 'CRON_TZ=${IANA_TIME_ZONE}' or 'TZ=${IANA_TIME_ZONE}'. The ${IANA_TIME_ZONE} may only be a valid string from IANA time zone database. For example, CRON_TZ=America/New_York 1 * * * *, or TZ=America/New_York 1 * * * *. This field is required for RECURRING tasks.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// The first run of the task will be after this time. If not specified, the task will run shortly after being submitted if ON_DEMAND and based on the schedule if RECURRING.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Trigger type of the user-specified Task
    /// Possible values are: `ON_DEMAND`, `RECURRING`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskTriggerSpec {
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
                    "disabled",
                    &self.r#disabled,
                ),
                to_pulumi_object_field(
                    "max_retries",
                    &self.r#max_retries,
                ),
                to_pulumi_object_field(
                    "schedule",
                    &self.r#schedule,
                ),
                to_pulumi_object_field(
                    "start_time",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskTriggerSpec {
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
                    r#disabled: {
                        let field_value = match fields_map.get("disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_retries: {
                        let field_value = match fields_map.get("max_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#start_time: {
                        let field_value = match fields_map.get("start_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
