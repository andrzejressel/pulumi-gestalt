#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParameters {
    /// The parameters for using an AWS Batch job as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "batchJobParameters")]
    pub r#batch_job_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersBatchJobParameters>>,
    /// The parameters for using an CloudWatch Logs log stream as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsParameters")]
    pub r#cloudwatch_logs_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersCloudwatchLogsParameters>>,
    /// The parameters for using an Amazon ECS task as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "ecsTaskParameters")]
    pub r#ecs_task_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersEcsTaskParameters>>,
    /// The parameters for using an EventBridge event bus as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "eventbridgeEventBusParameters")]
    pub r#eventbridge_event_bus_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersEventbridgeEventBusParameters>>,
    /// These are custom parameter to be used when the target is an API Gateway REST APIs or EventBridge ApiDestinations. Detailed below.
    #[builder(into)]
    #[serde(rename = "httpParameters")]
    pub r#http_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersHttpParameters>>,
    /// Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. Maximum length of 8192 characters.
    #[builder(into)]
    #[serde(rename = "inputTemplate")]
    pub r#input_template: Option<String>,
    /// The parameters for using a Kinesis stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "kinesisStreamParameters")]
    pub r#kinesis_stream_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersKinesisStreamParameters>>,
    /// The parameters for using a Lambda function as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "lambdaFunctionParameters")]
    pub r#lambda_function_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersLambdaFunctionParameters>>,
    /// These are custom parameters to be used when the target is a Amazon Redshift cluster to invoke the Amazon Redshift Data API BatchExecuteStatement. Detailed below.
    #[builder(into)]
    #[serde(rename = "redshiftDataParameters")]
    pub r#redshift_data_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersRedshiftDataParameters>>,
    /// The parameters for using a SageMaker pipeline as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "sagemakerPipelineParameters")]
    pub r#sagemaker_pipeline_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersSagemakerPipelineParameters>>,
    /// The parameters for using a Amazon SQS stream as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "sqsQueueParameters")]
    pub r#sqs_queue_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersSqsQueueParameters>>,
    /// The parameters for using a Step Functions state machine as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "stepFunctionStateMachineParameters")]
    pub r#step_function_state_machine_parameters: Option<Box<super::super::types::pipes::PipeTargetParametersStepFunctionStateMachineParameters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeTargetParameters {
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
                "batch_job_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_job_parameters,
                )
                .await,
            );
            map.insert(
                "cloudwatch_logs_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_logs_parameters,
                )
                .await,
            );
            map.insert(
                "ecs_task_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ecs_task_parameters,
                )
                .await,
            );
            map.insert(
                "eventbridge_event_bus_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#eventbridge_event_bus_parameters,
                )
                .await,
            );
            map.insert(
                "http_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_parameters,
                )
                .await,
            );
            map.insert(
                "input_template".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_template,
                )
                .await,
            );
            map.insert(
                "kinesis_stream_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_stream_parameters,
                )
                .await,
            );
            map.insert(
                "lambda_function_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lambda_function_parameters,
                )
                .await,
            );
            map.insert(
                "redshift_data_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redshift_data_parameters,
                )
                .await,
            );
            map.insert(
                "sagemaker_pipeline_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sagemaker_pipeline_parameters,
                )
                .await,
            );
            map.insert(
                "sqs_queue_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sqs_queue_parameters,
                )
                .await,
            );
            map.insert(
                "step_function_state_machine_parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#step_function_state_machine_parameters,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeTargetParameters {
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
                    r#batch_job_parameters: {
                        let field_value = match fields_map.get("batch_job_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_job_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logs_parameters: {
                        let field_value = match fields_map.get("cloudwatch_logs_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logs_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ecs_task_parameters: {
                        let field_value = match fields_map.get("ecs_task_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'ecs_task_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#eventbridge_event_bus_parameters: {
                        let field_value = match fields_map.get("eventbridge_event_bus_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'eventbridge_event_bus_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_parameters: {
                        let field_value = match fields_map.get("http_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_template: {
                        let field_value = match fields_map.get("input_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream_parameters: {
                        let field_value = match fields_map.get("kinesis_stream_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_stream_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_function_parameters: {
                        let field_value = match fields_map.get("lambda_function_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_function_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redshift_data_parameters: {
                        let field_value = match fields_map.get("redshift_data_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'redshift_data_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sagemaker_pipeline_parameters: {
                        let field_value = match fields_map.get("sagemaker_pipeline_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'sagemaker_pipeline_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqs_queue_parameters: {
                        let field_value = match fields_map.get("sqs_queue_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs_queue_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#step_function_state_machine_parameters: {
                        let field_value = match fields_map.get("step_function_state_machine_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_function_state_machine_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
