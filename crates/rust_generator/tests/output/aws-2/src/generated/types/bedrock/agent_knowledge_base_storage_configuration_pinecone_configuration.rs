#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentKnowledgeBaseStorageConfigurationPineconeConfiguration {
    /// Endpoint URL for your index management page.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: String,
    /// ARN of the secret that you created in AWS Secrets Manager that is linked to your Pinecone API key.
    #[builder(into)]
    #[serde(rename = "credentialsSecretArn")]
    pub r#credentials_secret_arn: String,
    /// The names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Option<Box<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationPineconeConfigurationFieldMapping>>,
    /// Namespace to be used to write new data to your database.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentKnowledgeBaseStorageConfigurationPineconeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "connection_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_string,
                )
                .await,
            );
            map.insert(
                "credentials_secret_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#credentials_secret_arn,
                )
                .await,
            );
            map.insert(
                "field_mapping".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_mapping,
                )
                .await,
            );
            map.insert(
                "namespace".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#namespace,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentKnowledgeBaseStorageConfigurationPineconeConfiguration {
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
                    r#connection_string: {
                        let field_value = match fields_map.get("connection_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#credentials_secret_arn: {
                        let field_value = match fields_map.get("credentials_secret_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'credentials_secret_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_mapping: {
                        let field_value = match fields_map.get("field_mapping") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_mapping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespace: {
                        let field_value = match fields_map.get("namespace") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
