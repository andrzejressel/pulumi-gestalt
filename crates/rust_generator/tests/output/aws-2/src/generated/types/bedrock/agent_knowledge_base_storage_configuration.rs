#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentKnowledgeBaseStorageConfiguration {
    /// The storage configuration of the knowledge base in Amazon OpenSearch Service. See `opensearch_serverless_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "opensearchServerlessConfiguration")]
    pub r#opensearch_serverless_configuration: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationOpensearchServerlessConfiguration>>,
    /// The storage configuration of the knowledge base in Pinecone. See `pinecone_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "pineconeConfiguration")]
    pub r#pinecone_configuration: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationPineconeConfiguration>>,
    /// Details about the storage configuration of the knowledge base in Amazon RDS. For more information, see [Create a vector index in Amazon RDS](https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup.html). See `rds_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "rdsConfiguration")]
    pub r#rds_configuration: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRdsConfiguration>>,
    /// The storage configuration of the knowledge base in Redis Enterprise Cloud. See `redis_enterprise_cloud_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "redisEnterpriseCloudConfiguration")]
    pub r#redis_enterprise_cloud_configuration: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration>>,
    /// Vector store service in which the knowledge base is stored. Valid Values: `OPENSEARCH_SERVERLESS`, `PINECONE`, `REDIS_ENTERPRISE_CLOUD`, `RDS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentKnowledgeBaseStorageConfiguration {
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
                    "opensearch_serverless_configuration",
                    &self.r#opensearch_serverless_configuration,
                ),
                to_pulumi_object_field(
                    "pinecone_configuration",
                    &self.r#pinecone_configuration,
                ),
                to_pulumi_object_field(
                    "rds_configuration",
                    &self.r#rds_configuration,
                ),
                to_pulumi_object_field(
                    "redis_enterprise_cloud_configuration",
                    &self.r#redis_enterprise_cloud_configuration,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentKnowledgeBaseStorageConfiguration {
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
                    r#opensearch_serverless_configuration: {
                        let field_value = match fields_map.get("opensearch_serverless_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearch_serverless_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinecone_configuration: {
                        let field_value = match fields_map.get("pinecone_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinecone_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rds_configuration: {
                        let field_value = match fields_map.get("rds_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'rds_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redis_enterprise_cloud_configuration: {
                        let field_value = match fields_map.get("redis_enterprise_cloud_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'redis_enterprise_cloud_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
