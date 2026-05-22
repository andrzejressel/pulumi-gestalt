#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineAssessmentSchedule {
    /// What day of the week the assessment will be run. Possible values are `Friday`, `Monday`, `Saturday`, `Sunday`, `Thursday`, `Tuesday` and `Wednesday`.
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: String,
    /// How many months between assessment runs. Valid values are between `1` and `5`.
    /// 
    /// > **NOTE:** Either one of `weekly_interval` or `monthly_occurrence` must be specified.
    #[builder(into)]
    #[serde(rename = "monthlyOccurrence")]
    pub r#monthly_occurrence: Option<i32>,
    /// What time the assessment will be run. Must be in the format `HH:mm`.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
    /// How many weeks between assessment runs. Valid values are between `1` and `6`.
    #[builder(into)]
    #[serde(rename = "weeklyInterval")]
    pub r#weekly_interval: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VirtualMachineAssessmentSchedule {
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
                    "dayOfWeek",
                    &self.r#day_of_week,
                ),
                to_pulumi_object_field(
                    "monthlyOccurrence",
                    &self.r#monthly_occurrence,
                ),
                to_pulumi_object_field(
                    "startTime",
                    &self.r#start_time,
                ),
                to_pulumi_object_field(
                    "weeklyInterval",
                    &self.r#weekly_interval,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VirtualMachineAssessmentSchedule {
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
                    r#day_of_week: {
                        let field_value = match fields_map.get("dayOfWeek") {
                            Some(value) => value,
                            None => bail!("Missing field 'dayOfWeek' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monthly_occurrence: {
                        let field_value = match fields_map.get("monthlyOccurrence") {
                            Some(value) => value,
                            None => bail!("Missing field 'monthlyOccurrence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_time: {
                        let field_value = match fields_map.get("startTime") {
                            Some(value) => value,
                            None => bail!("Missing field 'startTime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#weekly_interval: {
                        let field_value = match fields_map.get("weeklyInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'weeklyInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
