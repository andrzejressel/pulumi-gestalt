#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParameters {
    /// The parameters for using an Active MQ broker as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "activemqBrokerParameters")]
    pub r#activemq_broker_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersActivemqBrokerParameters>>,
    /// The parameters for using a DynamoDB stream as a source.  Detailed below.
    #[builder(into)]
    #[serde(rename = "dynamodbStreamParameters")]
    pub r#dynamodb_stream_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersDynamodbStreamParameters>>,
    /// The collection of event patterns used to [filter events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes-event-filtering.html). Detailed below.
    #[builder(into)]
    #[serde(rename = "filterCriteria")]
    pub r#filter_criteria: Option<Box<super::super::types::pipes::PipeSourceParametersFilterCriteria>>,
    /// The parameters for using a Kinesis stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "kinesisStreamParameters")]
    pub r#kinesis_stream_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersKinesisStreamParameters>>,
    /// The parameters for using an MSK stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "managedStreamingKafkaParameters")]
    pub r#managed_streaming_kafka_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersManagedStreamingKafkaParameters>>,
    /// The parameters for using a Rabbit MQ broker as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "rabbitmqBrokerParameters")]
    pub r#rabbitmq_broker_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersRabbitmqBrokerParameters>>,
    /// The parameters for using a self-managed Apache Kafka stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "selfManagedKafkaParameters")]
    pub r#self_managed_kafka_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersSelfManagedKafkaParameters>>,
    /// The parameters for using a Amazon SQS stream as a source. Detailed below.
    #[builder(into)]
    #[serde(rename = "sqsQueueParameters")]
    pub r#sqs_queue_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersSqsQueueParameters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "activemq_broker_parameters",
                    &self.r#activemq_broker_parameters,
                ),
                to_pulumi_object_field(
                    "dynamodb_stream_parameters",
                    &self.r#dynamodb_stream_parameters,
                ),
                to_pulumi_object_field(
                    "filter_criteria",
                    &self.r#filter_criteria,
                ),
                to_pulumi_object_field(
                    "kinesis_stream_parameters",
                    &self.r#kinesis_stream_parameters,
                ),
                to_pulumi_object_field(
                    "managed_streaming_kafka_parameters",
                    &self.r#managed_streaming_kafka_parameters,
                ),
                to_pulumi_object_field(
                    "rabbitmq_broker_parameters",
                    &self.r#rabbitmq_broker_parameters,
                ),
                to_pulumi_object_field(
                    "self_managed_kafka_parameters",
                    &self.r#self_managed_kafka_parameters,
                ),
                to_pulumi_object_field(
                    "sqs_queue_parameters",
                    &self.r#sqs_queue_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PipeSourceParameters {
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
                    r#activemq_broker_parameters: {
                        let field_value = match fields_map.get("activemq_broker_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'activemq_broker_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamodb_stream_parameters: {
                        let field_value = match fields_map.get("dynamodb_stream_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamodb_stream_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_criteria: {
                        let field_value = match fields_map.get("filter_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#managed_streaming_kafka_parameters: {
                        let field_value = match fields_map.get("managed_streaming_kafka_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_streaming_kafka_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rabbitmq_broker_parameters: {
                        let field_value = match fields_map.get("rabbitmq_broker_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'rabbitmq_broker_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_managed_kafka_parameters: {
                        let field_value = match fields_map.get("self_managed_kafka_parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_managed_kafka_parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
