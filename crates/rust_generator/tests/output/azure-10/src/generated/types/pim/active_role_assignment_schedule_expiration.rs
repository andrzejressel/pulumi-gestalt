#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActiveRoleAssignmentScheduleExpiration {
    /// The duration of the role assignment in days. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "durationDays")]
    pub r#duration_days: Option<i32>,
    /// The duration of the role assignment in hours. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "durationHours")]
    pub r#duration_hours: Option<i32>,
    /// The end date/time of the role assignment. Changing this forces a new resource to be created.
    /// 
    /// > Note: Only one of `duration_days`, `duration_hours` or `end_date_time` should be specified.
    #[builder(into)]
    #[serde(rename = "endDateTime")]
    pub r#end_date_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ActiveRoleAssignmentScheduleExpiration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "duration_days",
                    &self.r#duration_days,
                ),
                to_pulumi_object_field(
                    "duration_hours",
                    &self.r#duration_hours,
                ),
                to_pulumi_object_field(
                    "end_date_time",
                    &self.r#end_date_time,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ActiveRoleAssignmentScheduleExpiration {
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
                    r#duration_days: {
                        let field_value = match fields_map.get("duration_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration_hours: {
                        let field_value = match fields_map.get("duration_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_date_time: {
                        let field_value = match fields_map.get("end_date_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_date_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
