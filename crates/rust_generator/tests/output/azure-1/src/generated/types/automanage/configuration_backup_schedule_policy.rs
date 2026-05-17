#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationBackupSchedulePolicy {
    /// The schedule policy type of the backup policy. Possible value is `SimpleSchedulePolicy`. Defaults to `SimpleSchedulePolicy`.
    #[builder(into)]
    #[serde(rename = "schedulePolicyType")]
    pub r#schedule_policy_type: Option<String>,
    /// The schedule run days of the backup policy. Possible values are `Sunday`, `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday` and `Saturday`.
    #[builder(into)]
    #[serde(rename = "scheduleRunDays")]
    pub r#schedule_run_days: Option<Vec<String>>,
    /// The schedule run frequency of the backup policy. Possible values are `Daily` and `Weekly`. Defaults to `Daily`.
    #[builder(into)]
    #[serde(rename = "scheduleRunFrequency")]
    pub r#schedule_run_frequency: Option<String>,
    /// The schedule run times of the backup policy.
    #[builder(into)]
    #[serde(rename = "scheduleRunTimes")]
    pub r#schedule_run_times: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationBackupSchedulePolicy {
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
                    "schedule_policy_type",
                    &self.r#schedule_policy_type,
                ),
                to_pulumi_object_field(
                    "schedule_run_days",
                    &self.r#schedule_run_days,
                ),
                to_pulumi_object_field(
                    "schedule_run_frequency",
                    &self.r#schedule_run_frequency,
                ),
                to_pulumi_object_field(
                    "schedule_run_times",
                    &self.r#schedule_run_times,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationBackupSchedulePolicy {
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
                    r#schedule_policy_type: {
                        let field_value = match fields_map.get("schedule_policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_run_days: {
                        let field_value = match fields_map.get("schedule_run_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_run_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_run_frequency: {
                        let field_value = match fields_map.get("schedule_run_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_run_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_run_times: {
                        let field_value = match fields_map.get("schedule_run_times") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_run_times' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
