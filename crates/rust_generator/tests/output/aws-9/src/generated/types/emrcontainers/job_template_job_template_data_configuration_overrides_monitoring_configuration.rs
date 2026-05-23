#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
    /// Monitoring configurations for CloudWatch.
    #[builder(into)]
    pub r#cloud_watch_monitoring_configuration: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationCloudWatchMonitoringConfiguration>>,
    /// Monitoring configurations for the persistent application UI.
    #[builder(into)]
    pub r#persistent_app_ui: Option<String>,
    /// Amazon S3 configuration for monitoring log publishing.
    #[builder(into)]
    pub r#s_3_monitoring_configuration: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationS3MonitoringConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
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
                    "cloudWatchMonitoringConfiguration",
                    &self.r#cloud_watch_monitoring_configuration,
                ),
                to_pulumi_object_field(
                    "persistentAppUi",
                    &self.r#persistent_app_ui,
                ),
                to_pulumi_object_field(
                    "s3MonitoringConfiguration",
                    &self.r#s_3_monitoring_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("cloudWatchMonitoringConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudWatchMonitoringConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#persistent_app_ui: {
                        let field_value = match fields_map.get("persistentAppUi") {
                            Some(value) => value,
                            None => bail!("Missing field 'persistentAppUi' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_monitoring_configuration: {
                        let field_value = match fields_map.get("s3MonitoringConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 's3MonitoringConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
