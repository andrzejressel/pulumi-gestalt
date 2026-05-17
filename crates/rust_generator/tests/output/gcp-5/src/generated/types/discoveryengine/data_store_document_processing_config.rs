#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataStoreDocumentProcessingConfig {
    /// Whether chunking mode is enabled.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "chunkingConfig")]
    pub r#chunking_config: Option<Box<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigChunkingConfig>>,
    /// Configurations for default Document parser. If not specified, this resource
    /// will be configured to use a default DigitalParsingConfig, and the default parsing
    /// config will be applied to all file types for Document parsing.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "defaultParsingConfig")]
    pub r#default_parsing_config: Option<Box<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigDefaultParsingConfig>>,
    /// (Output)
    /// The full resource name of the Document Processing Config. Format:
    /// `projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/documentProcessingConfig`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Map from file type to override the default parsing configuration based on the file type. Supported keys:
    #[builder(into)]
    #[serde(rename = "parsingConfigOverrides")]
    pub r#parsing_config_overrides: Option<Vec<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverride>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataStoreDocumentProcessingConfig {
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
                    "chunking_config",
                    &self.r#chunking_config,
                ),
                to_pulumi_object_field(
                    "default_parsing_config",
                    &self.r#default_parsing_config,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "parsing_config_overrides",
                    &self.r#parsing_config_overrides,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataStoreDocumentProcessingConfig {
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
                    r#chunking_config: {
                        let field_value = match fields_map.get("chunking_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'chunking_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_parsing_config: {
                        let field_value = match fields_map.get("default_parsing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_parsing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parsing_config_overrides: {
                        let field_value = match fields_map.get("parsing_config_overrides") {
                            Some(value) => value,
                            None => bail!("Missing field 'parsing_config_overrides' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
