#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerInspectJobStorageConfigTimespanConfig {
    /// When the job is started by a JobTrigger we will automatically figure out a valid startTime to avoid
    /// scanning files that have not been modified since the last time the JobTrigger executed. This will
    /// be based on the time of the execution of the last run of the JobTrigger or the timespan endTime
    /// used in the last run of the JobTrigger.
    #[builder(into)]
    #[serde(rename = "enableAutoPopulationOfTimespanConfig")]
    pub r#enable_auto_population_of_timespan_config: Option<bool>,
    /// Exclude files, tables, or rows newer than this value. If not set, no upper time limit is applied.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// Exclude files, tables, or rows older than this value. If not set, no lower time limit is applied.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// Specification of the field containing the timestamp of scanned items.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timestampField")]
    pub r#timestamp_field: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobStorageConfigTimespanConfigTimestampField>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionJobTriggerInspectJobStorageConfigTimespanConfig {
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
                "enable_auto_population_of_timespan_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_auto_population_of_timespan_config,
                )
                .await,
            );
            map.insert(
                "end_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#end_time,
                )
                .await,
            );
            map.insert(
                "start_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#start_time,
                )
                .await,
            );
            map.insert(
                "timestamp_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timestamp_field,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionJobTriggerInspectJobStorageConfigTimespanConfig {
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
                    r#enable_auto_population_of_timespan_config: {
                        let field_value = match fields_map.get("enable_auto_population_of_timespan_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_auto_population_of_timespan_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#end_time: {
                        let field_value = match fields_map.get("end_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'end_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#timestamp_field: {
                        let field_value = match fields_map.get("timestamp_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
