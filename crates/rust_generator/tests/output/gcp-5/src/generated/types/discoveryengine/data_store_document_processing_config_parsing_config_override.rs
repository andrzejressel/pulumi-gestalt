#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataStoreDocumentProcessingConfigParsingConfigOverride {
    /// Configurations applied to digital parser.
    #[builder(into)]
    #[serde(rename = "digitalParsingConfig")]
    pub r#digital_parsing_config: Option<Box<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideDigitalParsingConfig>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fileType")]
    pub r#file_type: String,
    /// Configurations applied to layout parser.
    #[builder(into)]
    #[serde(rename = "layoutParsingConfig")]
    pub r#layout_parsing_config: Option<Box<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideLayoutParsingConfig>>,
    /// Configurations applied to OCR parser. Currently it only applies to PDFs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ocrParsingConfig")]
    pub r#ocr_parsing_config: Option<Box<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideOcrParsingConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataStoreDocumentProcessingConfigParsingConfigOverride {
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
                    "digital_parsing_config",
                    &self.r#digital_parsing_config,
                ),
                to_pulumi_object_field(
                    "file_type",
                    &self.r#file_type,
                ),
                to_pulumi_object_field(
                    "layout_parsing_config",
                    &self.r#layout_parsing_config,
                ),
                to_pulumi_object_field(
                    "ocr_parsing_config",
                    &self.r#ocr_parsing_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataStoreDocumentProcessingConfigParsingConfigOverride {
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
                    r#digital_parsing_config: {
                        let field_value = match fields_map.get("digital_parsing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'digital_parsing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#layout_parsing_config: {
                        let field_value = match fields_map.get("layout_parsing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'layout_parsing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ocr_parsing_config: {
                        let field_value = match fields_map.get("ocr_parsing_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ocr_parsing_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
