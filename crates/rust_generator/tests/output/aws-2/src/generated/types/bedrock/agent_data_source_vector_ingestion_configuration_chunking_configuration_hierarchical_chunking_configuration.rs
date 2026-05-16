#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration {
    /// Maximum number of tokens to include in a chunk. Must contain two `level_configurations`. See `level_configurations` for details.
    #[builder(into)]
    #[serde(rename = "levelConfigurations")]
    pub r#level_configurations: Vec<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfigurationLevelConfiguration>,
    /// The number of tokens to repeat across chunks in the same layer.
    #[builder(into)]
    #[serde(rename = "overlapTokens")]
    pub r#overlap_tokens: f64,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("level_configurations".to_string(), self.r#level_configurations.to_pulumi_value().await);
            map.insert("overlap_tokens".to_string(), self.r#overlap_tokens.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration {
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
                    r#level_configurations: {
                        let field_value = match fields_map.get("level_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'level_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfigurationLevelConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#overlap_tokens: {
                        let field_value = match fields_map.get("overlap_tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'overlap_tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <f64 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
