#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleKafka {
    /// Properties of the Apache Kafka producer client. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/apache-kafka-rule-action.html).
    #[builder(into)]
    pub r#client_properties: std::collections::HashMap<String, String>,
    /// The ARN of Kafka action's VPC `aws.iot.TopicRuleDestination`.
    #[builder(into)]
    pub r#destination_arn: String,
    /// The list of Kafka headers that you specify. Nested arguments below.
    #[builder(into)]
    pub r#headers: Option<Vec<super::super::types::iot::TopicRuleKafkaHeader>>,
    /// The Kafka message key.
    #[builder(into)]
    pub r#key: Option<String>,
    /// The Kafka message partition.
    #[builder(into)]
    pub r#partition: Option<String>,
    /// The Kafka topic for messages to be sent to the Kafka broker.
    #[builder(into)]
    pub r#topic: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleKafka {
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
                    "clientProperties",
                    &self.r#client_properties,
                ),
                to_pulumi_object_field(
                    "destinationArn",
                    &self.r#destination_arn,
                ),
                to_pulumi_object_field(
                    "headers",
                    &self.r#headers,
                ),
                to_pulumi_object_field(
                    "key",
                    &self.r#key,
                ),
                to_pulumi_object_field(
                    "partition",
                    &self.r#partition,
                ),
                to_pulumi_object_field(
                    "topic",
                    &self.r#topic,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleKafka {
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
                    r#client_properties: {
                        let field_value = match fields_map.get("clientProperties") {
                            Some(value) => value,
                            None => bail!("Missing field 'clientProperties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_arn: {
                        let field_value = match fields_map.get("destinationArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#partition: {
                        let field_value = match fields_map.get("partition") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic: {
                        let field_value = match fields_map.get("topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
