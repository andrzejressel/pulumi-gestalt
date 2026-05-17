#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into)]
    #[serde(rename = "dagProcessors")]
    pub r#dag_processors: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigDagProcessor>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into)]
    #[serde(rename = "schedulers")]
    pub r#schedulers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigScheduler>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into)]
    #[serde(rename = "triggerers")]
    pub r#triggerers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigTriggerer>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServers")]
    pub r#web_servers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWebServer>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into)]
    #[serde(rename = "workers")]
    pub r#workers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWorker>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEnvironmentConfigWorkloadsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "dag_processors",
                    &self.r#dag_processors,
                ),
                to_pulumi_object_field(
                    "schedulers",
                    &self.r#schedulers,
                ),
                to_pulumi_object_field(
                    "triggerers",
                    &self.r#triggerers,
                ),
                to_pulumi_object_field(
                    "web_servers",
                    &self.r#web_servers,
                ),
                to_pulumi_object_field(
                    "workers",
                    &self.r#workers,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEnvironmentConfigWorkloadsConfig {
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
                    r#dag_processors: {
                        let field_value = match fields_map.get("dag_processors") {
                            Some(value) => value,
                            None => bail!("Missing field 'dag_processors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedulers: {
                        let field_value = match fields_map.get("schedulers") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedulers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#triggerers: {
                        let field_value = match fields_map.get("triggerers") {
                            Some(value) => value,
                            None => bail!("Missing field 'triggerers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_servers: {
                        let field_value = match fields_map.get("web_servers") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_servers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workers: {
                        let field_value = match fields_map.get("workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
