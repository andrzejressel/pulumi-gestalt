#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime {
    /// Hours of day in 24 hour format.
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: Option<i32>,
    /// Minutes of hour of day.
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: Option<i32>,
    /// Fractions of seconds in nanoseconds.
    #[builder(into)]
    #[serde(rename = "nanos")]
    pub r#nanos: Option<i32>,
    /// Seconds of minutes of the time.
    #[builder(into)]
    #[serde(rename = "seconds")]
    pub r#seconds: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime {
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
                "hours".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hours,
                )
                .await,
            );
            map.insert(
                "minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minutes,
                )
                .await,
            );
            map.insert(
                "nanos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nanos,
                )
                .await,
            );
            map.insert(
                "seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#seconds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackupPlanBackupScheduleRpoConfigExclusionWindowStartTime {
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
                    r#hours: {
                        let field_value = match fields_map.get("hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minutes: {
                        let field_value = match fields_map.get("minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nanos: {
                        let field_value = match fields_map.get("nanos") {
                            Some(value) => value,
                            None => bail!("Missing field 'nanos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#seconds: {
                        let field_value = match fields_map.get("seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
