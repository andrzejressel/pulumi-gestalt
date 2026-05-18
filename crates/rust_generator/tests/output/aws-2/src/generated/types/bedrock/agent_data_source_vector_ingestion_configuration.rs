#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AgentDataSourceVectorIngestionConfiguration {
    /// Details about how to chunk the documents in the data source. A chunk refers to an excerpt from a data source that is returned when the knowledge base that it belongs to is queried. See `chunking_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "chunkingConfiguration")]
    pub r#chunking_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationChunkingConfiguration>>,
    /// Configuration for custom transformation of data source documents.
    #[builder(into)]
    #[serde(rename = "customTransformationConfiguration")]
    pub r#custom_transformation_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationCustomTransformationConfiguration>>,
    /// Configuration for custom parsing of data source documents. See `parsing_configuration` block for details.
    #[builder(into)]
    #[serde(rename = "parsingConfiguration")]
    pub r#parsing_configuration: Option<Box<super::super::types::bedrock::AgentDataSourceVectorIngestionConfigurationParsingConfiguration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AgentDataSourceVectorIngestionConfiguration {
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
                    "chunking_configuration",
                    &self.r#chunking_configuration,
                ),
                to_pulumi_object_field(
                    "custom_transformation_configuration",
                    &self.r#custom_transformation_configuration,
                ),
                to_pulumi_object_field(
                    "parsing_configuration",
                    &self.r#parsing_configuration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AgentDataSourceVectorIngestionConfiguration {
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
                    r#chunking_configuration: {
                        let field_value = match fields_map.get("chunking_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'chunking_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_transformation_configuration: {
                        let field_value = match fields_map.get("custom_transformation_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_transformation_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parsing_configuration: {
                        let field_value = match fields_map.get("parsing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'parsing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
