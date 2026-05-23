#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeSourceParameters {
    /// The parameters for using an Active MQ broker as a source. Detailed below.
    #[builder(into)]
    pub r#activemq_broker_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersActivemqBrokerParameters>>,
    /// The parameters for using a DynamoDB stream as a source.  Detailed below.
    #[builder(into)]
    pub r#dynamodb_stream_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersDynamodbStreamParameters>>,
    /// The collection of event patterns used to [filter events](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes-event-filtering.html). Detailed below.
    #[builder(into)]
    pub r#filter_criteria: Option<Box<super::super::types::pipes::PipeSourceParametersFilterCriteria>>,
    /// The parameters for using a Kinesis stream as a source. Detailed below.
    #[builder(into)]
    pub r#kinesis_stream_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersKinesisStreamParameters>>,
    /// The parameters for using an MSK stream as a source. Detailed below.
    #[builder(into)]
    pub r#managed_streaming_kafka_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersManagedStreamingKafkaParameters>>,
    /// The parameters for using a Rabbit MQ broker as a source. Detailed below.
    #[builder(into)]
    pub r#rabbitmq_broker_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersRabbitmqBrokerParameters>>,
    /// The parameters for using a self-managed Apache Kafka stream as a source. Detailed below.
    #[builder(into)]
    pub r#self_managed_kafka_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersSelfManagedKafkaParameters>>,
    /// The parameters for using a Amazon SQS stream as a source. Detailed below.
    #[builder(into)]
    pub r#sqs_queue_parameters: Option<Box<super::super::types::pipes::PipeSourceParametersSqsQueueParameters>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PipeSourceParameters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "activemqBrokerParameters",
                    &self.r#activemq_broker_parameters,
                ),
                to_pulumi_object_field(
                    "dynamodbStreamParameters",
                    &self.r#dynamodb_stream_parameters,
                ),
                to_pulumi_object_field(
                    "filterCriteria",
                    &self.r#filter_criteria,
                ),
                to_pulumi_object_field(
                    "kinesisStreamParameters",
                    &self.r#kinesis_stream_parameters,
                ),
                to_pulumi_object_field(
                    "managedStreamingKafkaParameters",
                    &self.r#managed_streaming_kafka_parameters,
                ),
                to_pulumi_object_field(
                    "rabbitmqBrokerParameters",
                    &self.r#rabbitmq_broker_parameters,
                ),
                to_pulumi_object_field(
                    "selfManagedKafkaParameters",
                    &self.r#self_managed_kafka_parameters,
                ),
                to_pulumi_object_field(
                    "sqsQueueParameters",
                    &self.r#sqs_queue_parameters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("activemqBrokerParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'activemqBrokerParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamodb_stream_parameters: {
                        let field_value = match fields_map.get("dynamodbStreamParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamodbStreamParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_criteria: {
                        let field_value = match fields_map.get("filterCriteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'filterCriteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream_parameters: {
                        let field_value = match fields_map.get("kinesisStreamParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisStreamParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_streaming_kafka_parameters: {
                        let field_value = match fields_map.get("managedStreamingKafkaParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedStreamingKafkaParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rabbitmq_broker_parameters: {
                        let field_value = match fields_map.get("rabbitmqBrokerParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'rabbitmqBrokerParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_managed_kafka_parameters: {
                        let field_value = match fields_map.get("selfManagedKafkaParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'selfManagedKafkaParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sqs_queue_parameters: {
                        let field_value = match fields_map.get("sqsQueueParameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'sqsQueueParameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
