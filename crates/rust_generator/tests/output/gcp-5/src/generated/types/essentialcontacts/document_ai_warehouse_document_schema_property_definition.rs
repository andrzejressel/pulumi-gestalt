#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinition {
    /// Date time property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "dateTimeTypeOptions")]
    pub r#date_time_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionDateTimeTypeOptions>>,
    /// The display-name for the property, used for front-end.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// Enum/categorical property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "enumTypeOptions")]
    pub r#enum_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionEnumTypeOptions>>,
    /// Float property.
    #[builder(into)]
    #[serde(rename = "floatTypeOptions")]
    pub r#float_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionFloatTypeOptions>>,
    /// Integer property.
    #[builder(into)]
    #[serde(rename = "integerTypeOptions")]
    pub r#integer_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionIntegerTypeOptions>>,
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
    pub r#map_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionMapTypeOptions>>,
    /// The name of the metadata property.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Nested structured data property.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "propertyTypeOptions")]
    pub r#property_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionPropertyTypeOptions>>,
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
    pub r#text_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTextTypeOptions>>,
    /// Timestamp property. Not supported by CMEK compliant deployment.
    #[builder(into)]
    #[serde(rename = "timestampTypeOptions")]
    pub r#timestamp_type_options: Option<Box<super::super::types::essentialcontacts::DocumentAiWarehouseDocumentSchemaPropertyDefinitionTimestampTypeOptions>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinition {
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
                "date_time_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#date_time_type_options,
                )
                .await,
            );
            map.insert(
                "display_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#display_name,
                )
                .await,
            );
            map.insert(
                "enum_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enum_type_options,
                )
                .await,
            );
            map.insert(
                "float_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#float_type_options,
                )
                .await,
            );
            map.insert(
                "integer_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#integer_type_options,
                )
                .await,
            );
            map.insert(
                "is_filterable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_filterable,
                )
                .await,
            );
            map.insert(
                "is_metadata".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_metadata,
                )
                .await,
            );
            map.insert(
                "is_repeatable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_repeatable,
                )
                .await,
            );
            map.insert(
                "is_required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_required,
                )
                .await,
            );
            map.insert(
                "is_searchable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#is_searchable,
                )
                .await,
            );
            map.insert(
                "map_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#map_type_options,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "property_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#property_type_options,
                )
                .await,
            );
            map.insert(
                "retrieval_importance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#retrieval_importance,
                )
                .await,
            );
            map.insert(
                "schema_sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_sources,
                )
                .await,
            );
            map.insert(
                "text_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#text_type_options,
                )
                .await,
            );
            map.insert(
                "timestamp_type_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timestamp_type_options,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DocumentAiWarehouseDocumentSchemaPropertyDefinition {
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
                    r#property_type_options: {
                        let field_value = match fields_map.get("property_type_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'property_type_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
