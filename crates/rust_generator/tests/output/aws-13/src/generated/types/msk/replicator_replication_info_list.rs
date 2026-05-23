#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicatorReplicationInfoList {
    /// Configuration relating to consumer group replication.
    #[builder(into)]
    #[serde(rename = "consumerGroupReplications")]
    pub r#consumer_group_replications: Vec<super::super::types::msk::ReplicatorReplicationInfoListConsumerGroupReplication>,
    #[builder(into)]
    #[serde(rename = "sourceKafkaClusterAlias")]
    pub r#source_kafka_cluster_alias: Option<String>,
    /// The ARN of the source Kafka cluster.
    #[builder(into)]
    #[serde(rename = "sourceKafkaClusterArn")]
    pub r#source_kafka_cluster_arn: String,
    /// The type of compression to use writing records to target Kafka cluster.
    #[builder(into)]
    #[serde(rename = "targetCompressionType")]
    pub r#target_compression_type: String,
    #[builder(into)]
    #[serde(rename = "targetKafkaClusterAlias")]
    pub r#target_kafka_cluster_alias: Option<String>,
    /// The ARN of the target Kafka cluster.
    #[builder(into)]
    #[serde(rename = "targetKafkaClusterArn")]
    pub r#target_kafka_cluster_arn: String,
    /// Configuration relating to topic replication.
    #[builder(into)]
    #[serde(rename = "topicReplications")]
    pub r#topic_replications: Vec<super::super::types::msk::ReplicatorReplicationInfoListTopicReplication>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicatorReplicationInfoList {
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
                    "consumerGroupReplications",
                    &self.r#consumer_group_replications,
                ),
                to_pulumi_object_field(
                    "sourceKafkaClusterAlias",
                    &self.r#source_kafka_cluster_alias,
                ),
                to_pulumi_object_field(
                    "sourceKafkaClusterArn",
                    &self.r#source_kafka_cluster_arn,
                ),
                to_pulumi_object_field(
                    "targetCompressionType",
                    &self.r#target_compression_type,
                ),
                to_pulumi_object_field(
                    "targetKafkaClusterAlias",
                    &self.r#target_kafka_cluster_alias,
                ),
                to_pulumi_object_field(
                    "targetKafkaClusterArn",
                    &self.r#target_kafka_cluster_arn,
                ),
                to_pulumi_object_field(
                    "topicReplications",
                    &self.r#topic_replications,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicatorReplicationInfoList {
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
                    r#consumer_group_replications: {
                        let field_value = match fields_map.get("consumerGroupReplications") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumerGroupReplications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_kafka_cluster_alias: {
                        let field_value = match fields_map.get("sourceKafkaClusterAlias") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceKafkaClusterAlias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_kafka_cluster_arn: {
                        let field_value = match fields_map.get("sourceKafkaClusterArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceKafkaClusterArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_compression_type: {
                        let field_value = match fields_map.get("targetCompressionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetCompressionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_kafka_cluster_alias: {
                        let field_value = match fields_map.get("targetKafkaClusterAlias") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetKafkaClusterAlias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_kafka_cluster_arn: {
                        let field_value = match fields_map.get("targetKafkaClusterArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'targetKafkaClusterArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_replications: {
                        let field_value = match fields_map.get("topicReplications") {
                            Some(value) => value,
                            None => bail!("Missing field 'topicReplications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
