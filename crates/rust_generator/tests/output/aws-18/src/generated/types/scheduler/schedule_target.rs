#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScheduleTarget {
    /// ARN of the target of this schedule, such as a SQS queue or ECS cluster. For universal targets, this is a [Service ARN specific to the target service](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html#supported-universal-targets).
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Information about an Amazon SQS queue that EventBridge Scheduler uses as a dead-letter queue for your schedule. If specified, EventBridge Scheduler delivers failed events that could not be successfully delivered to a target to the queue. Detailed below.
    #[builder(into)]
    #[serde(rename = "deadLetterConfig")]
    pub r#dead_letter_config: Option<Box<super::super::types::scheduler::ScheduleTargetDeadLetterConfig>>,
    /// Templated target type for the Amazon ECS [`RunTask`](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_RunTask.html) API operation. Detailed below.
    #[builder(into)]
    #[serde(rename = "ecsParameters")]
    pub r#ecs_parameters: Option<Box<super::super::types::scheduler::ScheduleTargetEcsParameters>>,
    /// Templated target type for the EventBridge [`PutEvents`](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html) API operation. Detailed below.
    #[builder(into)]
    #[serde(rename = "eventbridgeParameters")]
    pub r#eventbridge_parameters: Option<Box<super::super::types::scheduler::ScheduleTargetEventbridgeParameters>>,
    /// Text, or well-formed JSON, passed to the target. Read more in [Universal target](https://docs.aws.amazon.com/scheduler/latest/UserGuide/managing-targets-universal.html).
    #[builder(into)]
    #[serde(rename = "input")]
    pub r#input: Option<String>,
    /// Templated target type for the Amazon Kinesis [`PutRecord`](https://docs.aws.amazon.com/kinesis/latest/APIReference/API_PutRecord.html) API operation. Detailed below.
    #[builder(into)]
    #[serde(rename = "kinesisParameters")]
    pub r#kinesis_parameters: Option<Box<super::super::types::scheduler::ScheduleTargetKinesisParameters>>,
    /// Information about the retry policy settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Option<Box<super::super::types::scheduler::ScheduleTargetRetryPolicy>>,
    /// ARN of the IAM role that EventBridge Scheduler will use for this target when the schedule is invoked. Read more in [Set up the execution role](https://docs.aws.amazon.com/scheduler/latest/UserGuide/setting-up.html#setting-up-execution-role).
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// Templated target type for the Amazon SageMaker [`StartPipelineExecution`](https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_StartPipelineExecution.html) API operation. Detailed below.
    #[builder(into)]
    #[serde(rename = "sagemakerPipelineParameters")]
    pub r#sagemaker_pipeline_parameters: Option<Box<super::super::types::scheduler::ScheduleTargetSagemakerPipelineParameters>>,
    /// The templated target type for the Amazon SQS [`SendMessage`](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html) API operation. Detailed below.
    #[builder(into)]
    #[serde(rename = "sqsParameters")]
    pub r#sqs_parameters: Option<Box<super::super::types::scheduler::ScheduleTargetSqsParameters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ScheduleTarget {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("arn".to_string(), self.r#arn.to_pulumi_value().await);
            map.insert("dead_letter_config".to_string(), self.r#dead_letter_config.to_pulumi_value().await);
            map.insert("ecs_parameters".to_string(), self.r#ecs_parameters.to_pulumi_value().await);
            map.insert("eventbridge_parameters".to_string(), self.r#eventbridge_parameters.to_pulumi_value().await);
            map.insert("input".to_string(), self.r#input.to_pulumi_value().await);
            map.insert("kinesis_parameters".to_string(), self.r#kinesis_parameters.to_pulumi_value().await);
            map.insert("retry_policy".to_string(), self.r#retry_policy.to_pulumi_value().await);
            map.insert("role_arn".to_string(), self.r#role_arn.to_pulumi_value().await);
            map.insert("sagemaker_pipeline_parameters".to_string(), self.r#sagemaker_pipeline_parameters.to_pulumi_value().await);
            map.insert("sqs_parameters".to_string(), self.r#sqs_parameters.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ScheduleTarget {
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
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dead_letter_config: {
                        let field_value = match fields_map.get("dead_letter_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'dead_letter_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetDeadLetterConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ecs_parameters: {
                        let field_value = match fields_map.get("ecs_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecs_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetEcsParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#eventbridge_parameters: {
                        let field_value = match fields_map.get("eventbridge_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventbridge_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetEventbridgeParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input: {
                        let field_value = match fields_map.get("input") {
                            Some(value) => value,
                            None => bail!("Missing field 'input' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kinesis_parameters: {
                        let field_value = match fields_map.get("kinesis_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetKinesisParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#retry_policy: {
                        let field_value = match fields_map.get("retry_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'retry_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetRetryPolicy>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sagemaker_pipeline_parameters: {
                        let field_value = match fields_map.get("sagemaker_pipeline_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemaker_pipeline_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetSagemakerPipelineParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sqs_parameters: {
                        let field_value = match fields_map.get("sqs_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::scheduler::ScheduleTargetSqsParameters>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
