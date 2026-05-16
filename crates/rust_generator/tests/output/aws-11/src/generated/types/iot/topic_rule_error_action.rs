#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleErrorAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("cloudwatch_alarm".to_string(), self.r#cloudwatch_alarm.to_pulumi_value().await);
            map.insert("cloudwatch_logs".to_string(), self.r#cloudwatch_logs.to_pulumi_value().await);
            map.insert("cloudwatch_metric".to_string(), self.r#cloudwatch_metric.to_pulumi_value().await);
            map.insert("dynamodb".to_string(), self.r#dynamodb.to_pulumi_value().await);
            map.insert("dynamodbv_2".to_string(), self.r#dynamodbv_2.to_pulumi_value().await);
            map.insert("elasticsearch".to_string(), self.r#elasticsearch.to_pulumi_value().await);
            map.insert("firehose".to_string(), self.r#firehose.to_pulumi_value().await);
            map.insert("http".to_string(), self.r#http.to_pulumi_value().await);
            map.insert("iot_analytics".to_string(), self.r#iot_analytics.to_pulumi_value().await);
            map.insert("iot_events".to_string(), self.r#iot_events.to_pulumi_value().await);
            map.insert("kafka".to_string(), self.r#kafka.to_pulumi_value().await);
            map.insert("kinesis".to_string(), self.r#kinesis.to_pulumi_value().await);
            map.insert("lambda".to_string(), self.r#lambda.to_pulumi_value().await);
            map.insert("republish".to_string(), self.r#republish.to_pulumi_value().await);
            map.insert("s_3".to_string(), self.r#s_3.to_pulumi_value().await);
            map.insert("sns".to_string(), self.r#sns.to_pulumi_value().await);
            map.insert("sqs".to_string(), self.r#sqs.to_pulumi_value().await);
            map.insert("step_functions".to_string(), self.r#step_functions.to_pulumi_value().await);
            map.insert("timestream".to_string(), self.r#timestream.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleErrorAction {
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
                    r#cloudwatch_alarm: {
                        let field_value = match fields_map.get("cloudwatch_alarm") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_alarm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchAlarm>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_logs: {
                        let field_value = match fields_map.get("cloudwatch_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchLogs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cloudwatch_metric: {
                        let field_value = match fields_map.get("cloudwatch_metric") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_metric' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionCloudwatchMetric>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dynamodb: {
                        let field_value = match fields_map.get("dynamodb") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamodb' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionDynamodb>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dynamodbv_2: {
                        let field_value = match fields_map.get("dynamodbv_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamodbv_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionDynamodbv2>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#elasticsearch: {
                        let field_value = match fields_map.get("elasticsearch") {
                            Some(value) => value,
                            None => bail!("Missing field 'elasticsearch' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionElasticsearch>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#firehose: {
                        let field_value = match fields_map.get("firehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'firehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionFirehose>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http: {
                        let field_value = match fields_map.get("http") {
                            Some(value) => value,
                            None => bail!("Missing field 'http' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionHttp>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iot_analytics: {
                        let field_value = match fields_map.get("iot_analytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'iot_analytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionIotAnalytics>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#iot_events: {
                        let field_value = match fields_map.get("iot_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'iot_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionIotEvents>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kafka: {
                        let field_value = match fields_map.get("kafka") {
                            Some(value) => value,
                            None => bail!("Missing field 'kafka' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionKafka>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kinesis: {
                        let field_value = match fields_map.get("kinesis") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionKinesis>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda: {
                        let field_value = match fields_map.get("lambda") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionLambda>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#republish: {
                        let field_value = match fields_map.get("republish") {
                            Some(value) => value,
                            None => bail!("Missing field 'republish' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionRepublish>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionS3>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sns: {
                        let field_value = match fields_map.get("sns") {
                            Some(value) => value,
                            None => bail!("Missing field 'sns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionSns>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sqs: {
                        let field_value = match fields_map.get("sqs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionSqs>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#step_functions: {
                        let field_value = match fields_map.get("step_functions") {
                            Some(value) => value,
                            None => bail!("Missing field 'step_functions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionStepFunctions>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timestream: {
                        let field_value = match fields_map.get("timestream") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::iot::TopicRuleErrorActionTimestream>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
