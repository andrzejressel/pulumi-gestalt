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
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("opensearch_serverless_configuration".to_string(), self.r#opensearch_serverless_configuration.to_pulumi_value().await);
            map.insert("pinecone_configuration".to_string(), self.r#pinecone_configuration.to_pulumi_value().await);
            map.insert("rds_configuration".to_string(), self.r#rds_configuration.to_pulumi_value().await);
            map.insert("redis_enterprise_cloud_configuration".to_string(), self.r#redis_enterprise_cloud_configuration.to_pulumi_value().await);
            map.insert("type_".to_string(), self.r#type_.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentKnowledgeBaseStorageConfiguration {
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
                    r#opensearch_serverless_configuration: {
                        let field_value = match fields_map.get("opensearch_serverless_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'opensearch_serverless_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationOpensearchServerlessConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#pinecone_configuration: {
                        let field_value = match fields_map.get("pinecone_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinecone_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationPineconeConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#rds_configuration: {
                        let field_value = match fields_map.get("rds_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'rds_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRdsConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#redis_enterprise_cloud_configuration: {
                        let field_value = match fields_map.get("redis_enterprise_cloud_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'redis_enterprise_cloud_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
