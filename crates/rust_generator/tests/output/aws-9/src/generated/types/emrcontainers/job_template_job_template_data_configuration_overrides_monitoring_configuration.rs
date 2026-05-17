#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
    /// Monitoring configurations for CloudWatch.
    #[builder(into)]
    #[serde(rename = "cloudWatchMonitoringConfiguration")]
    pub r#cloud_watch_monitoring_configuration: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationCloudWatchMonitoringConfiguration>>,
    /// Monitoring configurations for the persistent application UI.
    #[builder(into)]
    #[serde(rename = "persistentAppUi")]
    pub r#persistent_app_ui: Option<String>,
    /// Amazon S3 configuration for monitoring log publishing.
    #[builder(into)]
    #[serde(rename = "s3MonitoringConfiguration")]
    pub r#s_3_monitoring_configuration: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationS3MonitoringConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
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
                "cloud_watch_monitoring_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_watch_monitoring_configuration,
                )
                .await,
            );
            map.insert(
                "persistent_app_ui".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#persistent_app_ui,
                )
                .await,
            );
            map.insert(
                "s_3_monitoring_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_monitoring_configuration,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
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
                    r#cloud_watch_monitoring_configuration: {
                        let field_value = match fields_map.get("cloud_watch_monitoring_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_watch_monitoring_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#persistent_app_ui: {
                        let field_value = match fields_map.get("persistent_app_ui") {
                            Some(value) => value,
                            None => bail!("Missing field 'persistent_app_ui' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_monitoring_configuration: {
                        let field_value = match fields_map.get("s_3_monitoring_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_monitoring_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
