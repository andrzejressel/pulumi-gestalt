#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinition {
    /// Date time property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    pub r#date_time_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionDateTimeTypeOptions>>,
    /// The display-name for the property, used for front-end.
    #[builder(into)]
    pub r#display_name: Option<String>,
    /// Enum/categorical property.
    /// Structure is documented below.
    #[builder(into)]
    pub r#enum_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionEnumTypeOptions>>,
    /// Float property.
    #[builder(into)]
    pub r#float_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionFloatTypeOptions>>,
    /// Integer property.
    #[builder(into)]
    pub r#integer_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionIntegerTypeOptions>>,
    /// Whether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable.
    #[builder(into)]
    pub r#is_filterable: Option<bool>,
    /// Whether the property is user supplied metadata.
    #[builder(into)]
    pub r#is_metadata: Option<bool>,
    /// Whether the property can have multiple values.
    #[builder(into)]
    pub r#is_repeatable: Option<bool>,
    /// Whether the property is mandatory.
    #[builder(into)]
    pub r#is_required: Option<bool>,
    /// Indicates that the property should be included in a global search.
    #[builder(into)]
    pub r#is_searchable: Option<bool>,
    /// Map property.
    #[builder(into)]
    pub r#map_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionMapTypeOptions>>,
    /// The name of the metadata property.
    #[builder(into)]
    pub r#name: String,
    /// Stores the retrieval importance.
    /// Possible values are: `HIGHEST`, `HIGHER`, `HIGH`, `MEDIUM`, `LOW`, `LOWEST`.
    #[builder(into)]
    pub r#retrieval_importance: Option<String>,
    /// The schema source information.
    /// Structure is documented below.
    #[builder(into)]
    pub r#schema_sources: Option<Vec<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionSchemaSource>>,
    /// Text property.
    #[builder(into)]
    pub r#text_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptionsPropertyDefinitionTextTypeOptions>>,
    /// Timestamp property. Not supported by CMEK compliant deployment.
    #[builder(into)]
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
                    "dateTimeTypeOptions",
                    &self.r#date_time_type_options,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "enumTypeOptions",
                    &self.r#enum_type_options,
                ),
                to_pulumi_object_field(
                    "floatTypeOptions",
                    &self.r#float_type_options,
                ),
                to_pulumi_object_field(
                    "integerTypeOptions",
                    &self.r#integer_type_options,
                ),
                to_pulumi_object_field(
                    "isFilterable",
                    &self.r#is_filterable,
                ),
                to_pulumi_object_field(
                    "isMetadata",
                    &self.r#is_metadata,
                ),
                to_pulumi_object_field(
                    "isRepeatable",
                    &self.r#is_repeatable,
                ),
                to_pulumi_object_field(
                    "isRequired",
                    &self.r#is_required,
                ),
                to_pulumi_object_field(
                    "isSearchable",
                    &self.r#is_searchable,
                ),
                to_pulumi_object_field(
                    "mapTypeOptions",
                    &self.r#map_type_options,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "retrievalImportance",
                    &self.r#retrieval_importance,
                ),
                to_pulumi_object_field(
                    "schemaSources",
                    &self.r#schema_sources,
                ),
                to_pulumi_object_field(
                    "textTypeOptions",
                    &self.r#text_type_options,
                ),
                to_pulumi_object_field(
                    "timestampTypeOptions",
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
                        let field_value = match fields_map.get("dateTimeTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'dateTimeTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enum_type_options: {
                        let field_value = match fields_map.get("enumTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'enumTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#float_type_options: {
                        let field_value = match fields_map.get("floatTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'floatTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#integer_type_options: {
                        let field_value = match fields_map.get("integerTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'integerTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_filterable: {
                        let field_value = match fields_map.get("isFilterable") {
                            Some(value) => value,
                            None => bail!("Missing field 'isFilterable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_metadata: {
                        let field_value = match fields_map.get("isMetadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'isMetadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_repeatable: {
                        let field_value = match fields_map.get("isRepeatable") {
                            Some(value) => value,
                            None => bail!("Missing field 'isRepeatable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_required: {
                        let field_value = match fields_map.get("isRequired") {
                            Some(value) => value,
                            None => bail!("Missing field 'isRequired' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_searchable: {
                        let field_value = match fields_map.get("isSearchable") {
                            Some(value) => value,
                            None => bail!("Missing field 'isSearchable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#map_type_options: {
                        let field_value = match fields_map.get("mapTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'mapTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("retrievalImportance") {
                            Some(value) => value,
                            None => bail!("Missing field 'retrievalImportance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_sources: {
                        let field_value = match fields_map.get("schemaSources") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemaSources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#text_type_options: {
                        let field_value = match fields_map.get("textTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'textTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_type_options: {
                        let field_value = match fields_map.get("timestampTypeOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampTypeOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
