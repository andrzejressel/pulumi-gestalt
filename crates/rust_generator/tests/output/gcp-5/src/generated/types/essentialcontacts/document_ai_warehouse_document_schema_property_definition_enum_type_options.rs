#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionEnumTypeOptions {
    /// List of possible enum values.
    #[builder(into)]
    #[serde(rename = "possibleValues")]
    pub r#possible_values: Vec<String>,
    /// Make sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "validationCheckDisabled")]
    pub r#validation_check_disabled: Option<bool>,
}
