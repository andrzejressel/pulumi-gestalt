#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into)]
    #[serde(rename = "dagProcessor")]
    pub r#dag_processor: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigDagProcessor>>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into)]
    #[serde(rename = "scheduler")]
    pub r#scheduler: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigScheduler>>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into)]
    #[serde(rename = "triggerer")]
    pub r#triggerer: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigTriggerer>>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServer")]
    pub r#web_server: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigWebServer>>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into)]
    #[serde(rename = "worker")]
    pub r#worker: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigWorker>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EnvironmentConfigWorkloadsConfig {
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
                    "dag_processor",
                    &self.r#dag_processor,
                ),
                to_pulumi_object_field(
                    "scheduler",
                    &self.r#scheduler,
                ),
                to_pulumi_object_field(
                    "triggerer",
                    &self.r#triggerer,
                ),
                to_pulumi_object_field(
                    "web_server",
                    &self.r#web_server,
                ),
                to_pulumi_object_field(
                    "worker",
                    &self.r#worker,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EnvironmentConfigWorkloadsConfig {
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
                    r#dag_processor: {
                        let field_value = match fields_map.get("dag_processor") {
                            Some(value) => value,
                            None => bail!("Missing field 'dag_processor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scheduler: {
                        let field_value = match fields_map.get("scheduler") {
                            Some(value) => value,
                            None => bail!("Missing field 'scheduler' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#triggerer: {
                        let field_value = match fields_map.get("triggerer") {
                            Some(value) => value,
                            None => bail!("Missing field 'triggerer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#web_server: {
                        let field_value = match fields_map.get("web_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'web_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker: {
                        let field_value = match fields_map.get("worker") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
