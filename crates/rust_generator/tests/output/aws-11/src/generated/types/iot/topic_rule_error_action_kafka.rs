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
