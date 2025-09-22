#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataStoreDocumentProcessingConfigParsingConfigOverride {
    /// Configurations applied to digital parser.
    #[builder(into)]
    #[serde(rename = "digitalParsingConfig")]
    pub r#digital_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideDigitalParsingConfig>>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "fileType")]
    pub r#file_type: String,
    /// Configurations applied to layout parser.
    #[builder(into)]
    #[serde(rename = "layoutParsingConfig")]
    pub r#layout_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideLayoutParsingConfig>>,
    /// Configurations applied to OCR parser. Currently it only applies to PDFs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ocrParsingConfig")]
    pub r#ocr_parsing_config: Box<Option<super::super::types::discoveryengine::DataStoreDocumentProcessingConfigParsingConfigOverrideOcrParsingConfig>>,
}
