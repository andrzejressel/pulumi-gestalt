#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParameters {
    /// The parameters for using an AWS Batch job as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "batchJobParameters")]
    pub r#batch_job_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersBatchJobParameters>>,
    /// The parameters for using an CloudWatch Logs log stream as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsParameters")]
    pub r#cloudwatch_logs_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersCloudwatchLogsParameters>>,
    /// The parameters for using an Amazon ECS task as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "ecsTaskParameters")]
    pub r#ecs_task_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersEcsTaskParameters>>,
    /// The parameters for using an EventBridge event bus as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "eventbridgeEventBusParameters")]
    pub r#eventbridge_event_bus_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersEventbridgeEventBusParameters>>,
    /// These are custom parameter to be used when the target is an API Gateway REST APIs or EventBridge ApiDestinations. Detailed below.
    #[builder(into)]
    #[serde(rename = "httpParameters")]
    pub r#http_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersHttpParameters>>,
    /// Valid JSON text passed to the target. In this case, nothing from the event itself is passed to the target. Maximum length of 8192 characters.
    #[builder(into)]
    #[serde(rename = "inputTemplate")]
    pub r#input_template: Option<String>,
    /// The parameters for using a Kinesis stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "kinesisStreamParameters")]
    pub r#kinesis_stream_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersKinesisStreamParameters>>,
    /// The parameters for using a Lambda function as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "lambdaFunctionParameters")]
    pub r#lambda_function_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersLambdaFunctionParameters>>,
    /// These are custom parameters to be used when the target is a Amazon Redshift cluster to invoke the Amazon Redshift Data API BatchExecuteStatement. Detailed below.
    #[builder(into)]
    #[serde(rename = "redshiftDataParameters")]
    pub r#redshift_data_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersRedshiftDataParameters>>,
    /// The parameters for using a SageMaker pipeline as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "sagemakerPipelineParameters")]
    pub r#sagemaker_pipeline_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersSagemakerPipelineParameters>>,
    /// The parameters for using a Amazon SQS stream as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "sqsQueueParameters")]
    pub r#sqs_queue_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersSqsQueueParameters>>,
    /// The parameters for using a Step Functions state machine as a target. Detailed below.
    #[builder(into)]
    #[serde(rename = "stepFunctionStateMachineParameters")]
    pub r#step_function_state_machine_parameters: Box<Option<super::super::types::pipes::PipeTargetParametersStepFunctionStateMachineParameters>>,
}
