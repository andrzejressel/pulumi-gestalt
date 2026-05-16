#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleErrorActionKafka {
    /// Properties of the Apache Kafka producer client. For more info, see the [AWS documentation](https://docs.aws.amazon.com/iot/latest/developerguide/apache-kafka-rule-action.html).
    #[builder(into)]
    #[serde(rename = "clientProperties")]
    pub r#client_properties: std::collections::HashMap<String, String>,
    /// The ARN of Kafka action's VPC `aws.iot.TopicRuleDestination`.
    #[builder(into)]
    #[serde(rename = "destinationArn")]
    pub r#destination_arn: String,
    /// The list of Kafka headers that you specify. Nested arguments below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::iot::TopicRuleErrorActionKafkaHeader>>,
    /// The Kafka message key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The Kafka message partition.
    #[builder(into)]
    #[serde(rename = "partition")]
    pub r#partition: Option<String>,
    /// The Kafka topic for messages to be sent to the Kafka broker.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TopicRuleErrorActionKafka {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("client_properties".to_string(), self.r#client_properties.to_pulumi_value().await);
            map.insert("destination_arn".to_string(), self.r#destination_arn.to_pulumi_value().await);
            map.insert("headers".to_string(), self.r#headers.to_pulumi_value().await);
            map.insert("key".to_string(), self.r#key.to_pulumi_value().await);
            map.insert("partition".to_string(), self.r#partition.to_pulumi_value().await);
            map.insert("topic".to_string(), self.r#topic.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TopicRuleErrorActionKafka {
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
                    r#client_properties: {
                        let field_value = match fields_map.get("client_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <std::collections::HashMap<String, String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#destination_arn: {
                        let field_value = match fields_map.get("destination_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::iot::TopicRuleErrorActionKafkaHeader>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#key: {
                        let field_value = match fields_map.get("key") {
                            Some(value) => value,
                            None => bail!("Missing field 'key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#partition: {
                        let field_value = match fields_map.get("partition") {
                            Some(value) => value,
                            None => bail!("Missing field 'partition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#topic: {
                        let field_value = match fields_map.get("topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
