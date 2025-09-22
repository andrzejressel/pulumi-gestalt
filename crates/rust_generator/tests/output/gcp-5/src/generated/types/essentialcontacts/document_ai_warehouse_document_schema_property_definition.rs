#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinition {
    /// Date time property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "dateTimeTypeOptions")]
    pub r#date_time_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionDateTimeTypeOptions>>,
    /// The display-name for the property, used for front-end.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Enum/categorical property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enumTypeOptions")]
    pub r#enum_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionEnumTypeOptions>>,
    /// Float property.
    #[builder(into)]
    #[serde(rename = "floatTypeOptions")]
    pub r#float_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionFloatTypeOptions>>,
    /// Integer property.
    #[builder(into)]
    #[serde(rename = "integerTypeOptions")]
    pub r#integer_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionIntegerTypeOptions>>,
    /// Whether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable.
    #[builder(into)]
    #[serde(rename = "isFilterable")]
    pub r#is_filterable: Option<bool>,
    /// Whether the property is user supplied metadata.
    #[builder(into)]
    #[serde(rename = "isMetadata")]
    pub r#is_metadata: Option<bool>,
    /// Whether the property can have multiple values.
    #[builder(into)]
    #[serde(rename = "isRepeatable")]
    pub r#is_repeatable: Option<bool>,
    /// Whether the property is mandatory.
    #[builder(into)]
    #[serde(rename = "isRequired")]
    pub r#is_required: Option<bool>,
    /// Indicates that the property should be included in a global search.
    #[builder(into)]
    #[serde(rename = "isSearchable")]
    pub r#is_searchable: Option<bool>,
    /// Map property.
    #[builder(into)]
    #[serde(rename = "mapTypeOptions")]
    pub r#map_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionMapTypeOptions>>,
    /// The name of the metadata property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Nested structured data property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "propertyTypeOptions")]
    pub r#property_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptions>>,
    /// Stores the retrieval importance.
    /// Possible values are: `HIGHEST`, `HIGHER`, `HIGH`, `MEDIUM`, `LOW`, `LOWEST`.
    #[builder(into)]
    #[serde(rename = "retrievalImportance")]
    pub r#retrieval_importance: Option<String>,
    /// The schema source information.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schemaSources")]
    pub r#schema_sources: Option<Vec<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionSchemaSource>>,
    /// Text property.
    #[builder(into)]
    #[serde(rename = "textTypeOptions")]
    pub r#text_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTextTypeOptions>>,
    /// Timestamp property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "timestampTypeOptions")]
    pub r#timestamp_type_options: Box<Option<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTimestampTypeOptions>>,
}
