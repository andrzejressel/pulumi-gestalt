#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogTableStorageDescriptor {
    /// List of locations that point to the path where a Delta table is located.
    #[builder(into)]
    #[serde(rename = "additionalLocations")]
    pub r#additional_locations: Option<Vec<String>>,
    /// List of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into)]
    #[serde(rename = "bucketColumns")]
    pub r#bucket_columns: Option<Vec<String>>,
    /// Configuration block for columns in the table. See `columns` below.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Option<Vec<super::super::types::glue::CatalogTableStorageDescriptorColumn>>,
    /// Whether the data in the table is compressed.
    #[builder(into)]
    #[serde(rename = "compressed")]
    pub r#compressed: Option<bool>,
    /// Input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Option<String>,
    /// Physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Must be specified if the table contains any dimension columns.
    #[builder(into)]
    #[serde(rename = "numberOfBuckets")]
    pub r#number_of_buckets: Option<i32>,
    /// Output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Option<String>,
    /// User-supplied properties in key-value form.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Object that references a schema stored in the AWS Glue Schema Registry. When creating a table, you can pass an empty list of columns for the schema, and instead use a schema reference. See Schema Reference below.
    #[builder(into)]
    #[serde(rename = "schemaReference")]
    pub r#schema_reference: Option<Box<super::super::types::glue::CatalogTableStorageDescriptorSchemaReference>>,
    /// Configuration block for serialization and deserialization ("SerDe") information. See `ser_de_info` below.
    #[builder(into)]
    #[serde(rename = "serDeInfo")]
    pub r#ser_de_info: Option<Box<super::super::types::glue::CatalogTableStorageDescriptorSerDeInfo>>,
    /// Configuration block with information about values that appear very frequently in a column (skewed values). See `skewed_info` below.
    #[builder(into)]
    #[serde(rename = "skewedInfo")]
    pub r#skewed_info: Option<Box<super::super::types::glue::CatalogTableStorageDescriptorSkewedInfo>>,
    /// Configuration block for the sort order of each bucket in the table. See `sort_columns` below.
    #[builder(into)]
    #[serde(rename = "sortColumns")]
    pub r#sort_columns: Option<Vec<super::super::types::glue::CatalogTableStorageDescriptorSortColumn>>,
    /// Whether the table data is stored in subdirectories.
    #[builder(into)]
    #[serde(rename = "storedAsSubDirectories")]
    pub r#stored_as_sub_directories: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CatalogTableStorageDescriptor {
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
                "additional_locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#additional_locations,
                )
                .await,
            );
            map.insert(
                "bucket_columns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_columns,
                )
                .await,
            );
            map.insert(
                "columns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#columns,
                )
                .await,
            );
            map.insert(
                "compressed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compressed,
                )
                .await,
            );
            map.insert(
                "input_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_format,
                )
                .await,
            );
            map.insert(
                "location".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location,
                )
                .await,
            );
            map.insert(
                "number_of_buckets".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_buckets,
                )
                .await,
            );
            map.insert(
                "output_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_format,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "schema_reference".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schema_reference,
                )
                .await,
            );
            map.insert(
                "ser_de_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ser_de_info,
                )
                .await,
            );
            map.insert(
                "skewed_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#skewed_info,
                )
                .await,
            );
            map.insert(
                "sort_columns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sort_columns,
                )
                .await,
            );
            map.insert(
                "stored_as_sub_directories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#stored_as_sub_directories,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CatalogTableStorageDescriptor {
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
                        let field_value = match fields_map.get("additional_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_columns: {
                        let field_value = match fields_map.get("bucket_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("input_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("number_of_buckets") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_buckets' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_format: {
                        let field_value = match fields_map.get("output_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#schema_reference: {
                        let field_value = match fields_map.get("schema_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ser_de_info: {
                        let field_value = match fields_map.get("ser_de_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'ser_de_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skewed_info: {
                        let field_value = match fields_map.get("skewed_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewed_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sort_columns: {
                        let field_value = match fields_map.get("sort_columns") {
                            Some(value) => value,
                            None => bail!("Missing field 'sort_columns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stored_as_sub_directories: {
                        let field_value = match fields_map.get("stored_as_sub_directories") {
                            Some(value) => value,
                            None => bail!("Missing field 'stored_as_sub_directories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
