#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PartitionStorageDescriptor {
    /// A list of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into)]
    pub r#bucket_columns: Option<Vec<String>>,
    /// A list of the Columns in the table.
    #[builder(into)]
    pub r#columns: Option<Vec<super::super::types::glue::PartitionStorageDescriptorColumn>>,
    /// True if the data in the table is compressed, or False if not.
    #[builder(into)]
    pub r#compressed: Option<bool>,
    /// The input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into)]
    pub r#input_format: Option<String>,
    /// The physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into)]
    pub r#location: Option<String>,
    /// Must be specified if the table contains any dimension columns.
    #[builder(into)]
    pub r#number_of_buckets: Option<i32>,
    /// The output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into)]
    pub r#output_format: Option<String>,
    /// User-supplied properties in key-value form.
    #[builder(into)]
    pub r#parameters: Option<std::collections::BTreeMap<String, String>>,
    /// Serialization/deserialization (SerDe) information.
    #[builder(into)]
    pub r#ser_de_info: Option<Box<super::super::types::glue::PartitionStorageDescriptorSerDeInfo>>,
    /// Information about values that appear very frequently in a column (skewed values).
    #[builder(into)]
    pub r#skewed_info: Option<Box<super::super::types::glue::PartitionStorageDescriptorSkewedInfo>>,
    /// A list of Order objects specifying the sort order of each bucket in the table.
    #[builder(into)]
    pub r#sort_columns: Option<Vec<super::super::types::glue::PartitionStorageDescriptorSortColumn>>,
    /// True if the table data is stored in subdirectories, or False if not.
    #[builder(into)]
    pub r#stored_as_sub_directories: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PartitionStorageDescriptor {
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
                    "serDeInfo",
                    &self.r#ser_de_info,
                ),
                to_pulumi_object_field(
                    "skewedInfo",
                    &self.r#skewed_info,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PartitionStorageDescriptor {
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
                    r#ser_de_info: {
                        let field_value = match fields_map.get("serDeInfo") {
                            Some(value) => value,
                            None => bail!("Missing field 'serDeInfo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skewed_info: {
                        let field_value = match fields_map.get("skewedInfo") {
                            Some(value) => value,
                            None => bail!("Missing field 'skewedInfo' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
