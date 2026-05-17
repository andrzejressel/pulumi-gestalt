#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RefreshScheduleSchedule {
    /// The type of refresh that the dataset undergoes. Valid values are `INCREMENTAL_REFRESH` and `FULL_REFRESH`.
    #[builder(into)]
    #[serde(rename = "refreshType")]
    pub r#refresh_type: String,
    /// The configuration of the [schedule frequency](https://docs.aws.amazon.com/quicksight/latest/APIReference/API_RefreshFrequency.html). See schedule_frequency.
    #[builder(into)]
    #[serde(rename = "scheduleFrequency")]
    pub r#schedule_frequency: Option<Box<super::super::types::quicksight::RefreshScheduleScheduleScheduleFrequency>>,
    /// Time after which the refresh schedule can be started, expressed in `YYYY-MM-DDTHH:MM:SS` format.
    #[builder(into)]
    #[serde(rename = "startAfterDateTime")]
    pub r#start_after_date_time: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RefreshScheduleSchedule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "refresh_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#refresh_type,
                )
                .await,
            );
            map.insert(
                "schedule_frequency".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule_frequency,
                )
                .await,
            );
            map.insert(
                "start_after_date_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_after_date_time,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RefreshScheduleSchedule {
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
                    r#refresh_type: {
                        let field_value = match fields_map.get("refresh_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_frequency: {
                        let field_value = match fields_map.get("schedule_frequency") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_frequency' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#start_after_date_time: {
                        let field_value = match fields_map.get("start_after_date_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'start_after_date_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
