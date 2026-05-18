#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinition {
    /// Date time property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "dateTimeTypeOptions")]
    pub r#date_time_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionDateTimeTypeOptions>>,
    /// The display-name for the property, used for front-end.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Enum/categorical property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enumTypeOptions")]
    pub r#enum_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionEnumTypeOptions>>,
    /// Float property.
    #[builder(into)]
    #[serde(rename = "floatTypeOptions")]
    pub r#float_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionFloatTypeOptions>>,
    /// Integer property.
    #[builder(into)]
    #[serde(rename = "integerTypeOptions")]
    pub r#integer_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionIntegerTypeOptions>>,
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
    pub r#map_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionMapTypeOptions>>,
    /// The name of the metadata property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Stores the retrieval importance.
    /// Possible values are: `HIGHEST`, `HIGHER`, `HIGH`, `MEDIUM`, `LOW`, `LOWEST`.
    #[builder(into)]
    #[serde(rename = "retrievalImportance")]
    pub r#retrieval_importance: Option<String>,
    /// The schema source information.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schemaSources")]
    pub r#schema_sources: Option<Vec<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionSchemaSource>>,
    /// Text property.
    #[builder(into)]
    #[serde(rename = "textTypeOptions")]
    pub r#text_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionTextTypeOptions>>,
    /// Timestamp property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "timestampTypeOptions")]
    pub r#timestamp_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionTimestampTypeOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinition {
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
                    "date_time_type_options",
                    &self.r#date_time_type_options,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "enum_type_options",
                    &self.r#enum_type_options,
                ),
                to_pulumi_object_field(
                    "float_type_options",
                    &self.r#float_type_options,
                ),
                to_pulumi_object_field(
                    "integer_type_options",
                    &self.r#integer_type_options,
                ),
                to_pulumi_object_field(
                    "is_filterable",
                    &self.r#is_filterable,
                ),
                to_pulumi_object_field(
                    "is_metadata",
                    &self.r#is_metadata,
                ),
                to_pulumi_object_field(
                    "is_repeatable",
                    &self.r#is_repeatable,
                ),
                to_pulumi_object_field(
                    "is_required",
                    &self.r#is_required,
                ),
                to_pulumi_object_field(
                    "is_searchable",
                    &self.r#is_searchable,
                ),
                to_pulumi_object_field(
                    "map_type_options",
                    &self.r#map_type_options,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "retrieval_importance",
                    &self.r#retrieval_importance,
                ),
                to_pulumi_object_field(
                    "schema_sources",
                    &self.r#schema_sources,
                ),
                to_pulumi_object_field(
                    "text_type_options",
                    &self.r#text_type_options,
                ),
                to_pulumi_object_field(
                    "timestamp_type_options",
                    &self.r#timestamp_type_options,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinition {
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
                    r#date_time_type_options: {
                        let field_value = match fields_map.get("date_time_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_time_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_type_options: {
                        let field_value = match fields_map.get("enum_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'enum_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#float_type_options: {
                        let field_value = match fields_map.get("float_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'float_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer_type_options: {
                        let field_value = match fields_map.get("integer_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'integer_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_filterable: {
                        let field_value = match fields_map.get("is_filterable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_filterable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_metadata: {
                        let field_value = match fields_map.get("is_metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_repeatable: {
                        let field_value = match fields_map.get("is_repeatable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_repeatable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_required: {
                        let field_value = match fields_map.get("is_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_searchable: {
                        let field_value = match fields_map.get("is_searchable") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_searchable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_type_options: {
                        let field_value = match fields_map.get("map_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'map_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#retrieval_importance: {
                        let field_value = match fields_map.get("retrieval_importance") {
                            Some(value) => value,
                            None => bail!("Missing field 'retrieval_importance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_sources: {
                        let field_value = match fields_map.get("schema_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_type_options: {
                        let field_value = match fields_map.get("text_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'text_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_type_options: {
                        let field_value = match fields_map.get("timestamp_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
