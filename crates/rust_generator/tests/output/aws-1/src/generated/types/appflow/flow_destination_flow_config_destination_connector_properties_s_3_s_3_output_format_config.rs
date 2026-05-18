#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig {
    /// Aggregation settings that you can use to customize the output format of your flow data. See Aggregation Config for more details.
    #[builder(into)]
    #[serde(rename = "aggregationConfig")]
    pub r#aggregation_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigAggregationConfig>>,
    /// File type that Amazon AppFlow places in the Amazon S3 bucket. Valid values are `CSV`, `JSON`, and `PARQUET`.
    #[builder(into)]
    #[serde(rename = "fileType")]
    pub r#file_type: Option<String>,
    /// Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date. See Prefix Config for more details.
    #[builder(into)]
    #[serde(rename = "prefixConfig")]
    pub r#prefix_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig>>,
    /// Whether the data types from the source system need to be preserved (Only valid for `Parquet` file type)
    #[builder(into)]
    #[serde(rename = "preserveSourceDataTyping")]
    pub r#preserve_source_data_typing: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig {
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
                    "aggregation_config",
                    &self.r#aggregation_config,
                ),
                to_pulumi_object_field(
                    "file_type",
                    &self.r#file_type,
                ),
                to_pulumi_object_field(
                    "prefix_config",
                    &self.r#prefix_config,
                ),
                to_pulumi_object_field(
                    "preserve_source_data_typing",
                    &self.r#preserve_source_data_typing,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfig {
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
                    r#aggregation_config: {
                        let field_value = match fields_map.get("aggregation_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'aggregation_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_type: {
                        let field_value = match fields_map.get("file_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_config: {
                        let field_value = match fields_map.get("prefix_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_source_data_typing: {
                        let field_value = match fields_map.get("preserve_source_data_typing") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_source_data_typing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
