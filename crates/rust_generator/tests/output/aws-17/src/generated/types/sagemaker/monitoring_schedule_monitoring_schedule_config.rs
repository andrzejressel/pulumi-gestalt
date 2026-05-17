#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MonitoringScheduleMonitoringScheduleConfig {
    /// The name of the monitoring job definition to schedule.
    #[builder(into)]
    #[serde(rename = "monitoringJobDefinitionName")]
    pub r#monitoring_job_definition_name: String,
    /// The type of the monitoring job definition to schedule. Valid values are `DataQuality`, `ModelQuality`, `ModelBias` or `ModelExplainability`
    #[builder(into)]
    #[serde(rename = "monitoringType")]
    pub r#monitoring_type: String,
    /// Configures the monitoring schedule. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "scheduleConfig")]
    pub r#schedule_config: Option<Box<super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfigScheduleConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MonitoringScheduleMonitoringScheduleConfig {
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
                "monitoring_job_definition_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitoring_job_definition_name,
                )
                .await,
            );
            map.insert(
                "monitoring_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitoring_type,
                )
                .await,
            );
            map.insert(
                "schedule_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedule_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MonitoringScheduleMonitoringScheduleConfig {
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
                    r#monitoring_job_definition_name: {
                        let field_value = match fields_map.get("monitoring_job_definition_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_job_definition_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitoring_type: {
                        let field_value = match fields_map.get("monitoring_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitoring_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedule_config: {
                        let field_value = match fields_map.get("schedule_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedule_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
