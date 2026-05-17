#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDefinitionHumanLoopConfig {
    /// The Amazon Resource Name (ARN) of the human task user interface.
    #[builder(into)]
    #[serde(rename = "humanTaskUiArn")]
    pub r#human_task_ui_arn: String,
    /// Defines the amount of money paid to an Amazon Mechanical Turk worker for each task performed. See Public Workforce Task Price details below.
    #[builder(into)]
    #[serde(rename = "publicWorkforceTaskPrice")]
    pub r#public_workforce_task_price: Option<Box<super::super::types::sagemaker::FlowDefinitionHumanLoopConfigPublicWorkforceTaskPrice>>,
    /// The length of time that a task remains available for review by human workers. Valid value range between `1` and `864000`.
    #[builder(into)]
    #[serde(rename = "taskAvailabilityLifetimeInSeconds")]
    pub r#task_availability_lifetime_in_seconds: Option<i32>,
    /// The number of distinct workers who will perform the same task on each object. Valid value range between `1` and `3`.
    #[builder(into)]
    #[serde(rename = "taskCount")]
    pub r#task_count: i32,
    /// A description for the human worker task.
    #[builder(into)]
    #[serde(rename = "taskDescription")]
    pub r#task_description: String,
    /// An array of keywords used to describe the task so that workers can discover the task.
    #[builder(into)]
    #[serde(rename = "taskKeywords")]
    pub r#task_keywords: Option<Vec<String>>,
    /// The amount of time that a worker has to complete a task. The default value is `3600` seconds.
    #[builder(into)]
    #[serde(rename = "taskTimeLimitInSeconds")]
    pub r#task_time_limit_in_seconds: Option<i32>,
    /// A title for the human worker task.
    #[builder(into)]
    #[serde(rename = "taskTitle")]
    pub r#task_title: String,
    /// The Amazon Resource Name (ARN) of the human task user interface. Amazon Resource Name (ARN) of a team of workers. For Public workforces see [AWS Docs](https://docs.aws.amazon.com/sagemaker/latest/dg/sms-workforce-management-public.html).
    #[builder(into)]
    #[serde(rename = "workteamArn")]
    pub r#workteam_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDefinitionHumanLoopConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "human_task_ui_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#human_task_ui_arn,
                )
                .await,
            );
            map.insert(
                "public_workforce_task_price".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#public_workforce_task_price,
                )
                .await,
            );
            map.insert(
                "task_availability_lifetime_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_availability_lifetime_in_seconds,
                )
                .await,
            );
            map.insert(
                "task_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_count,
                )
                .await,
            );
            map.insert(
                "task_description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_description,
                )
                .await,
            );
            map.insert(
                "task_keywords".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_keywords,
                )
                .await,
            );
            map.insert(
                "task_time_limit_in_seconds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_time_limit_in_seconds,
                )
                .await,
            );
            map.insert(
                "task_title".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#task_title,
                )
                .await,
            );
            map.insert(
                "workteam_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workteam_arn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDefinitionHumanLoopConfig {
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
                    r#human_task_ui_arn: {
                        let field_value = match fields_map.get("human_task_ui_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'human_task_ui_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#public_workforce_task_price: {
                        let field_value = match fields_map.get("public_workforce_task_price") {
                            Some(value) => value,
                            None => bail!("Missing field 'public_workforce_task_price' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_availability_lifetime_in_seconds: {
                        let field_value = match fields_map.get("task_availability_lifetime_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_availability_lifetime_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_count: {
                        let field_value = match fields_map.get("task_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_description: {
                        let field_value = match fields_map.get("task_description") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_keywords: {
                        let field_value = match fields_map.get("task_keywords") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_keywords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_time_limit_in_seconds: {
                        let field_value = match fields_map.get("task_time_limit_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_time_limit_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#task_title: {
                        let field_value = match fields_map.get("task_title") {
                            Some(value) => value,
                            None => bail!("Missing field 'task_title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workteam_arn: {
                        let field_value = match fields_map.get("workteam_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'workteam_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
