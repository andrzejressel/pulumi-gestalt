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
