#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAutoBackupManualSchedule {
    /// A list of days on which backup can take place. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`
    /// 
    /// > **NOTE:** `days_of_week` can only be specified when `manual_schedule` is set to `Weekly`
    #[builder(into)]
    pub r#days_of_weeks: Option<Vec<String>>,
    /// Frequency of full backups. Valid values include `Daily` or `Weekly`.
    #[builder(into)]
    pub r#full_backup_frequency: String,
    /// Start hour of a given day during which full backups can take place. Valid values are from `0` to `23`.
    #[builder(into)]
    pub r#full_backup_start_hour: i32,
    /// Duration of the time window of a given day during which full backups can take place, in hours. Valid values are between `1` and `23`.
    #[builder(into)]
    pub r#full_backup_window_in_hours: i32,
    /// Frequency of log backups, in minutes. Valid values are from `5` to `60`.
    #[builder(into)]
    pub r#log_backup_frequency_in_minutes: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAutoBackupManualSchedule {
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
                    "fullBackupFrequency",
                    &self.r#full_backup_frequency,
                ),
                to_pulumi_object_field(
                    "fullBackupStartHour",
                    &self.r#full_backup_start_hour,
                ),
                to_pulumi_object_field(
                    "fullBackupWindowInHours",
                    &self.r#full_backup_window_in_hours,
                ),
                to_pulumi_object_field(
                    "logBackupFrequencyInMinutes",
                    &self.r#log_backup_frequency_in_minutes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAutoBackupManualSchedule {
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
                    r#full_backup_frequency: {
                        let field_value = match fields_map.get("fullBackupFrequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'fullBackupFrequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_backup_start_hour: {
                        let field_value = match fields_map.get("fullBackupStartHour") {
                            Some(value) => value,
                            None => bail!("Missing field 'fullBackupStartHour' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#full_backup_window_in_hours: {
                        let field_value = match fields_map.get("fullBackupWindowInHours") {
                            Some(value) => value,
                            None => bail!("Missing field 'fullBackupWindowInHours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_backup_frequency_in_minutes: {
                        let field_value = match fields_map.get("logBackupFrequencyInMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'logBackupFrequencyInMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
