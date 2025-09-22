#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorAction {
    #[builder(into)]
    #[serde(rename = "cloudwatchAlarm")]
    pub r#cloudwatch_alarm: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchAlarm>>,
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchLogs>>,
    #[builder(into)]
    #[serde(rename = "cloudwatchMetric")]
    pub r#cloudwatch_metric: Box<Option<super::super::types::iot::TopicRuleErrorActionCloudwatchMetric>>,
    #[builder(into)]
    #[serde(rename = "dynamodb")]
    pub r#dynamodb: Box<Option<super::super::types::iot::TopicRuleErrorActionDynamodb>>,
    #[builder(into)]
    #[serde(rename = "dynamodbv2")]
    pub r#dynamodbv_2: Box<Option<super::super::types::iot::TopicRuleErrorActionDynamodbv2>>,
    #[builder(into)]
    #[serde(rename = "elasticsearch")]
    pub r#elasticsearch: Box<Option<super::super::types::iot::TopicRuleErrorActionElasticsearch>>,
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Box<Option<super::super::types::iot::TopicRuleErrorActionFirehose>>,
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Box<Option<super::super::types::iot::TopicRuleErrorActionHttp>>,
    #[builder(into)]
    #[serde(rename = "iotAnalytics")]
    pub r#iot_analytics: Box<Option<super::super::types::iot::TopicRuleErrorActionIotAnalytics>>,
    #[builder(into)]
    #[serde(rename = "iotEvents")]
    pub r#iot_events: Box<Option<super::super::types::iot::TopicRuleErrorActionIotEvents>>,
    #[builder(into)]
    #[serde(rename = "kafka")]
    pub r#kafka: Box<Option<super::super::types::iot::TopicRuleErrorActionKafka>>,
    #[builder(into)]
    #[serde(rename = "kinesis")]
    pub r#kinesis: Box<Option<super::super::types::iot::TopicRuleErrorActionKinesis>>,
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<Option<super::super::types::iot::TopicRuleErrorActionLambda>>,
    #[builder(into)]
    #[serde(rename = "republish")]
    pub r#republish: Box<Option<super::super::types::iot::TopicRuleErrorActionRepublish>>,
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::iot::TopicRuleErrorActionS3>>,
    #[builder(into)]
    #[serde(rename = "sns")]
    pub r#sns: Box<Option<super::super::types::iot::TopicRuleErrorActionSns>>,
    #[builder(into)]
    #[serde(rename = "sqs")]
    pub r#sqs: Box<Option<super::super::types::iot::TopicRuleErrorActionSqs>>,
    #[builder(into)]
    #[serde(rename = "stepFunctions")]
    pub r#step_functions: Box<Option<super::super::types::iot::TopicRuleErrorActionStepFunctions>>,
    #[builder(into)]
    #[serde(rename = "timestream")]
    pub r#timestream: Box<Option<super::super::types::iot::TopicRuleErrorActionTimestream>>,
}
