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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "consumer_group_replications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#consumer_group_replications,
                )
                .await,
            );
            map.insert(
                "source_kafka_cluster_alias".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_kafka_cluster_alias,
                )
                .await,
            );
            map.insert(
                "source_kafka_cluster_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_kafka_cluster_arn,
                )
                .await,
            );
            map.insert(
                "target_compression_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_compression_type,
                )
                .await,
            );
            map.insert(
                "target_kafka_cluster_alias".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_kafka_cluster_alias,
                )
                .await,
            );
            map.insert(
                "target_kafka_cluster_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_kafka_cluster_arn,
                )
                .await,
            );
            map.insert(
                "topic_replications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_replications,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("consumer_group_replications") {
                            Some(value) => value,
                            None => bail!("Missing field 'consumer_group_replications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_kafka_cluster_alias: {
                        let field_value = match fields_map.get("source_kafka_cluster_alias") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_kafka_cluster_alias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_kafka_cluster_arn: {
                        let field_value = match fields_map.get("source_kafka_cluster_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_kafka_cluster_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_compression_type: {
                        let field_value = match fields_map.get("target_compression_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_compression_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_kafka_cluster_alias: {
                        let field_value = match fields_map.get("target_kafka_cluster_alias") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_kafka_cluster_alias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_kafka_cluster_arn: {
                        let field_value = match fields_map.get("target_kafka_cluster_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_kafka_cluster_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_replications: {
                        let field_value = match fields_map.get("topic_replications") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_replications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
