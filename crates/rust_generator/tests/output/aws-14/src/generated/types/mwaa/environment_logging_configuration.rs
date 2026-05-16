#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentLoggingConfiguration {
    /// (Optional) Log configuration options for processing DAGs. See Module logging configuration for more information. Disabled by default.
    #[builder(into)]
    #[serde(rename = "dagProcessingLogs")]
    pub r#dag_processing_logs: Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationDagProcessingLogs>>,
    /// Log configuration options for the schedulers. See Module logging configuration for more information. Disabled by default.
    #[builder(into)]
    #[serde(rename = "schedulerLogs")]
    pub r#scheduler_logs: Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationSchedulerLogs>>,
    /// Log configuration options for DAG tasks. See Module logging configuration for more information. Enabled by default with `INFO` log level.
    #[builder(into)]
    #[serde(rename = "taskLogs")]
    pub r#task_logs: Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationTaskLogs>>,
    /// Log configuration options for the webservers. See Module logging configuration for more information. Disabled by default.
    #[builder(into)]
    #[serde(rename = "webserverLogs")]
    pub r#webserver_logs: Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationWebserverLogs>>,
    /// Log configuration options for the workers. See Module logging configuration for more information. Disabled by default.
    #[builder(into)]
    #[serde(rename = "workerLogs")]
    pub r#worker_logs: Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationWorkerLogs>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentLoggingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("dag_processing_logs".to_string(), self.r#dag_processing_logs.to_pulumi_value().await);
            map.insert("scheduler_logs".to_string(), self.r#scheduler_logs.to_pulumi_value().await);
            map.insert("task_logs".to_string(), self.r#task_logs.to_pulumi_value().await);
            map.insert("webserver_logs".to_string(), self.r#webserver_logs.to_pulumi_value().await);
            map.insert("worker_logs".to_string(), self.r#worker_logs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentLoggingConfiguration {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#dag_processing_logs: {
                        let field_value = match fields_map.get("dag_processing_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'dag_processing_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationDagProcessingLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#scheduler_logs: {
                        let field_value = match fields_map.get("scheduler_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduler_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationSchedulerLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#task_logs: {
                        let field_value = match fields_map.get("task_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationTaskLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#webserver_logs: {
                        let field_value = match fields_map.get("webserver_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'webserver_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationWebserverLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#worker_logs: {
                        let field_value = match fields_map.get("worker_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::mwaa::EnvironmentLoggingConfigurationWorkerLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
