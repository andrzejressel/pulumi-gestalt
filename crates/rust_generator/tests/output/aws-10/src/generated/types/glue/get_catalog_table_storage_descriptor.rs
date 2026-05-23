#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCatalogTableStorageDescriptor {
    /// List of locations that point to the path where a Delta table is located
    #[builder(into)]
    pub r#additional_locations: Vec<String>,
    /// List of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into)]
    pub r#bucket_columns: Vec<String>,
    /// Configuration block for columns in the table. See `columns` below.
    #[builder(into)]
    pub r#columns: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorColumn>,
    /// Whether the data in the table is compressed.
    #[builder(into)]
    pub r#compressed: bool,
    /// Input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into)]
    pub r#input_format: String,
    /// Physical location of the table. By default, this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into)]
    pub r#location: String,
    /// Is if the table contains any dimension columns.
    #[builder(into)]
    pub r#number_of_buckets: i32,
    /// Output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into)]
    pub r#output_format: String,
    /// Map of initialization parameters for the SerDe, in key-value form.
    #[builder(into)]
    pub r#parameters: std::collections::HashMap<String, String>,
    /// Object that references a schema stored in the AWS Glue Schema Registry. See `schema_reference` below.
    #[builder(into)]
    pub r#schema_references: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSchemaReference>,
    /// Configuration block for serialization and deserialization ("SerDe") information. See `ser_de_info` below.
    #[builder(into)]
    pub r#ser_de_infos: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSerDeInfo>,
    /// Configuration block with information about values that appear very frequently in a column (skewed values). See `skewed_info` below.
    #[builder(into)]
    pub r#skewed_infos: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSkewedInfo>,
    /// Configuration block for the sort order of each bucket in the table. See `sort_columns` below.
    #[builder(into)]
    pub r#sort_columns: Vec<super::super::types::glue::GetCatalogTableStorageDescriptorSortColumn>,
    /// Whether the table data is stored in subdirectories.
    #[builder(into)]
    pub r#stored_as_sub_directories: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetCatalogTableStorageDescriptor {
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
                    "additionalLocations",
                    &self.r#additional_locations,
                ),
                to_pulumi_object_field(
                    "bucketColumns",
                    &self.r#bucket_columns,
                ),
                to_pulumi_object_field(
                    "columns",
                    &self.r#columns,
                ),
                to_pulumi_object_field(
                    "compressed",
                    &self.r#compressed,
                ),
                to_pulumi_object_field(
                    "inputFormat",
                    &self.r#input_format,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "numberOfBuckets",
                    &self.r#number_of_buckets,
                ),
                to_pulumi_object_field(
                    "outputFormat",
                    &self.r#output_format,
                ),
                to_pulumi_object_field(
                    "parameters",
                    &self.r#parameters,
                ),
                to_pulumi_object_field(
                    "schemaReferences",
                    &self.r#schema_references,
                ),
                to_pulumi_object_field(
                    "serDeInfos",
                    &self.r#ser_de_infos,
                ),
                to_pulumi_object_field(
                    "skewedInfos",
                    &self.r#skewed_infos,
                ),
                to_pulumi_object_field(
                    "sortColumns",
                    &self.r#sort_columns,
                ),
                to_pulumi_object_field(
                    "storedAsSubDirectories",
                    &self.r#stored_as_sub_directories,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetCatalogTableStorageDescriptor {
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
                    r#additional_locations: {
                        let field_value = match fields_map.get("additionalLocations") {
                            Some(value) => value,
                            None => bail!("Missing field 'additionalLocations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_columns: {
                        let field_value = match fields_map.get("bucketColumns") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketColumns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#columns: {
                        let field_value = match fields_map.get("columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compressed: {
                        let field_value = match fields_map.get("compressed") {
                            Some(value) => value,
                            None => bail!("Missing field 'compressed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_format: {
                        let field_value = match fields_map.get("inputFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_buckets: {
                        let field_value = match fields_map.get("numberOfBuckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberOfBuckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_format: {
                        let field_value = match fields_map.get("outputFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_references: {
                        let field_value = match fields_map.get("schemaReferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemaReferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ser_de_infos: {
                        let field_value = match fields_map.get("serDeInfos") {
                            Some(value) => value,
                            None => bail!("Missing field 'serDeInfos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skewed_infos: {
                        let field_value = match fields_map.get("skewedInfos") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewedInfos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sort_columns: {
                        let field_value = match fields_map.get("sortColumns") {
                            Some(value) => value,
                            None => bail!("Missing field 'sortColumns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stored_as_sub_directories: {
                        let field_value = match fields_map.get("storedAsSubDirectories") {
                            Some(value) => value,
                            None => bail!("Missing field 'storedAsSubDirectories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
