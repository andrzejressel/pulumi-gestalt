#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorAction {
    #[builder(into)]
    #[serde(rename = "cloudwatchAlarm")]
    pub r#cloudwatch_alarm: Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchAlarm>>,
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchLogs>>,
    #[builder(into)]
    #[serde(rename = "cloudwatchMetric")]
    pub r#cloudwatch_metric: Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchMetric>>,
    #[builder(into)]
    #[serde(rename = "dynamodb")]
    pub r#dynamodb: Option<Box<super::super::types::iot::TopicRuleErrorActionDynamodb>>,
    #[builder(into)]
    #[serde(rename = "dynamodbv2")]
    pub r#dynamodbv_2: Option<Box<super::super::types::iot::TopicRuleErrorActionDynamodbv2>>,
    #[builder(into)]
    #[serde(rename = "elasticsearch")]
    pub r#elasticsearch: Option<Box<super::super::types::iot::TopicRuleErrorActionElasticsearch>>,
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Option<Box<super::super::types::iot::TopicRuleErrorActionFirehose>>,
    #[builder(into)]
    #[serde(rename = "http")]
    pub r#http: Option<Box<super::super::types::iot::TopicRuleErrorActionHttp>>,
    #[builder(into)]
    #[serde(rename = "iotAnalytics")]
    pub r#iot_analytics: Option<Box<super::super::types::iot::TopicRuleErrorActionIotAnalytics>>,
    #[builder(into)]
    #[serde(rename = "iotEvents")]
    pub r#iot_events: Option<Box<super::super::types::iot::TopicRuleErrorActionIotEvents>>,
    #[builder(into)]
    #[serde(rename = "kafka")]
    pub r#kafka: Option<Box<super::super::types::iot::TopicRuleErrorActionKafka>>,
    #[builder(into)]
    #[serde(rename = "kinesis")]
    pub r#kinesis: Option<Box<super::super::types::iot::TopicRuleErrorActionKinesis>>,
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Option<Box<super::super::types::iot::TopicRuleErrorActionLambda>>,
    #[builder(into)]
    #[serde(rename = "republish")]
    pub r#republish: Option<Box<super::super::types::iot::TopicRuleErrorActionRepublish>>,
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::iot::TopicRuleErrorActionS3>>,
    #[builder(into)]
    #[serde(rename = "sns")]
    pub r#sns: Option<Box<super::super::types::iot::TopicRuleErrorActionSns>>,
    #[builder(into)]
    #[serde(rename = "sqs")]
    pub r#sqs: Option<Box<super::super::types::iot::TopicRuleErrorActionSqs>>,
    #[builder(into)]
    #[serde(rename = "stepFunctions")]
    pub r#step_functions: Option<Box<super::super::types::iot::TopicRuleErrorActionStepFunctions>>,
    #[builder(into)]
    #[serde(rename = "timestream")]
    pub r#timestream: Option<Box<super::super::types::iot::TopicRuleErrorActionTimestream>>,
}
