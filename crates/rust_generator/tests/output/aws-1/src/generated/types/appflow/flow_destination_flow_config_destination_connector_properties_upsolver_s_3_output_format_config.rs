#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig {
    /// Aggregation settings that you can use to customize the output format of your flow data. See Aggregation Config for more details.
    #[builder(into)]
    #[serde(rename = "aggregationConfig")]
    pub r#aggregation_config: Option<Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigAggregationConfig>>,
    /// File type that Amazon AppFlow places in the Amazon S3 bucket. Valid values are `CSV`, `JSON`, and `PARQUET`.
    #[builder(into)]
    #[serde(rename = "fileType")]
    pub r#file_type: Option<String>,
    /// Determines the prefix that Amazon AppFlow applies to the folder name in the Amazon S3 bucket. You can name folders according to the flow frequency and date. See Prefix Config for more details.
    #[builder(into)]
    #[serde(rename = "prefixConfig")]
    pub r#prefix_config: Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfigPrefixConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "aggregation_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aggregation_config,
                )
                .await,
            );
            map.insert(
                "file_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_type,
                )
                .await,
            );
            map.insert(
                "prefix_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix_config,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolverS3OutputFormatConfig {
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
