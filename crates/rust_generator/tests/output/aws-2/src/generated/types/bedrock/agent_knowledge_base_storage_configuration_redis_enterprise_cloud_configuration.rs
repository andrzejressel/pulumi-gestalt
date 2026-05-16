#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration {
    /// ARN of the secret that you created in AWS Secrets Manager that is linked to your Redis Enterprise Cloud database.
    #[builder(into)]
    #[serde(rename = "credentialsSecretArn")]
    pub r#credentials_secret_arn: String,
    /// Endpoint URL of the Redis Enterprise Cloud database.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: String,
    /// The names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfigurationFieldMapping>>,
    /// Name of the vector index.
    #[builder(into)]
    #[serde(rename = "vectorIndexName")]
    pub r#vector_index_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("credentials_secret_arn".to_string(), self.r#credentials_secret_arn.to_pulumi_value().await);
            map.insert("endpoint".to_string(), self.r#endpoint.to_pulumi_value().await);
            map.insert("field_mapping".to_string(), self.r#field_mapping.to_pulumi_value().await);
            map.insert("vector_index_name".to_string(), self.r#vector_index_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration {
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
                    r#credentials_secret_arn: {
                        let field_value = match fields_map.get("credentials_secret_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials_secret_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#endpoint: {
                        let field_value = match fields_map.get("endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#field_mapping: {
                        let field_value = match fields_map.get("field_mapping") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_mapping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfigurationFieldMapping>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#vector_index_name: {
                        let field_value = match fields_map.get("vector_index_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'vector_index_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
