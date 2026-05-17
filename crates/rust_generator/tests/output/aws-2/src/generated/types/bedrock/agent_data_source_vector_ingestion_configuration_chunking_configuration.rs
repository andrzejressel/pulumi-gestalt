#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfigurationChunkingConfiguration {
    /// Option for chunking your source data, either in fixed-sized chunks or as one chunk. Valid values: `FIXED_SIZE`, `HIERARCHICAL`, `SEMANTIC`, `NONE`.
    #[builder(into)]
    #[serde(rename = "chunkingStrategy")]
    pub r#chunking_strategy: String,
    /// Configurations for when you choose fixed-size chunking. Requires chunking_strategy as `FIXED_SIZE`. See `fixed_size_chunking_configuration` for details.
    #[builder(into)]
    #[serde(rename = "fixedSizeChunkingConfiguration")]
    pub r#fixed_size_chunking_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationFixedSizeChunkingConfiguration>>,
    /// Configurations for when you choose hierarchical chunking. Requires chunking_strategy as `HIERARCHICAL`. See `hierarchical_chunking_configuration` for details.
    #[builder(into)]
    #[serde(rename = "hierarchicalChunkingConfiguration")]
    pub r#hierarchical_chunking_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationHierarchicalChunkingConfiguration>>,
    /// Configurations for when you choose semantic chunking. Requires chunking_strategy as `SEMANTIC`. See `semantic_chunking_configuration` for details.
    #[builder(into)]
    #[serde(rename = "semanticChunkingConfiguration")]
    pub r#semantic_chunking_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfigurationSemanticChunkingConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfiguration {
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
                    "chunking_strategy",
                    &self.r#chunking_strategy,
                ),
                to_pulumi_object_field(
                    "fixed_size_chunking_configuration",
                    &self.r#fixed_size_chunking_configuration,
                ),
                to_pulumi_object_field(
                    "hierarchical_chunking_configuration",
                    &self.r#hierarchical_chunking_configuration,
                ),
                to_pulumi_object_field(
                    "semantic_chunking_configuration",
                    &self.r#semantic_chunking_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfigurationChunkingConfiguration {
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
                    r#chunking_strategy: {
                        let field_value = match fields_map.get("chunking_strategy") {
                            Some(value) => value,
                            None => bail!("Missing field 'chunking_strategy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_size_chunking_configuration: {
                        let field_value = match fields_map.get("fixed_size_chunking_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_size_chunking_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hierarchical_chunking_configuration: {
                        let field_value = match fields_map.get("hierarchical_chunking_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'hierarchical_chunking_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#semantic_chunking_configuration: {
                        let field_value = match fields_map.get("semantic_chunking_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'semantic_chunking_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
