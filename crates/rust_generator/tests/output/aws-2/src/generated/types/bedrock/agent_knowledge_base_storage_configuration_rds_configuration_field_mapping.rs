#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentKnowledgeBaseStorageConfigurationRdsConfigurationFieldMapping {
    /// Name of the field in which Amazon Bedrock stores metadata about the vector store.
    #[builder(into)]
    #[serde(rename = "metadataField")]
    pub r#metadata_field: String,
    /// Name of the field in which Amazon Bedrock stores the ID for each entry.
    #[builder(into)]
    #[serde(rename = "primaryKeyField")]
    pub r#primary_key_field: String,
    /// Name of the field in which Amazon Bedrock stores the raw text from your data. The text is split according to the chunking strategy you choose.
    #[builder(into)]
    #[serde(rename = "textField")]
    pub r#text_field: String,
    /// Name of the field in which Amazon Bedrock stores the vector embeddings for your data sources.
    #[builder(into)]
    #[serde(rename = "vectorField")]
    pub r#vector_field: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentKnowledgeBaseStorageConfigurationRdsConfigurationFieldMapping {
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
                "metadata_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metadata_field,
                )
                .await,
            );
            map.insert(
                "primary_key_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_key_field,
                )
                .await,
            );
            map.insert(
                "text_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#text_field,
                )
                .await,
            );
            map.insert(
                "vector_field".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vector_field,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentKnowledgeBaseStorageConfigurationRdsConfigurationFieldMapping {
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
                    r#metadata_field: {
                        let field_value = match fields_map.get("metadata_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_key_field: {
                        let field_value = match fields_map.get("primary_key_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_key_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_field: {
                        let field_value = match fields_map.get("text_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vector_field: {
                        let field_value = match fields_map.get("vector_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'vector_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
