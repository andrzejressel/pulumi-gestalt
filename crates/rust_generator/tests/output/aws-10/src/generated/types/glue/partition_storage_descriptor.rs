#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PartitionStorageDescriptor {
    /// A list of reducer grouping columns, clustering columns, and bucketing columns in the table.
    #[builder(into)]
    #[serde(rename = "bucketColumns")]
    pub r#bucket_columns: Option<Vec<String>>,
    /// A list of the Columns in the table.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Option<Vec<super::super::types::glue::PartitionStorageDescriptorColumn>>,
    /// True if the data in the table is compressed, or False if not.
    #[builder(into)]
    #[serde(rename = "compressed")]
    pub r#compressed: Option<bool>,
    /// The input format: SequenceFileInputFormat (binary), or TextInputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: Option<String>,
    /// The physical location of the table. By default this takes the form of the warehouse location, followed by the database location in the warehouse, followed by the table name.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// Must be specified if the table contains any dimension columns.
    #[builder(into)]
    #[serde(rename = "numberOfBuckets")]
    pub r#number_of_buckets: Option<i32>,
    /// The output format: SequenceFileOutputFormat (binary), or IgnoreKeyTextOutputFormat, or a custom format.
    #[builder(into)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Option<String>,
    /// User-supplied properties in key-value form.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// Serialization/deserialization (SerDe) information.
    #[builder(into)]
    #[serde(rename = "serDeInfo")]
    pub r#ser_de_info: Box<Option<super::super::types::glue::PartitionStorageDescriptorSerDeInfo>>,
    /// Information about values that appear very frequently in a column (skewed values).
    #[builder(into)]
    #[serde(rename = "skewedInfo")]
    pub r#skewed_info: Box<Option<super::super::types::glue::PartitionStorageDescriptorSkewedInfo>>,
    /// A list of Order objects specifying the sort order of each bucket in the table.
    #[builder(into)]
    #[serde(rename = "sortColumns")]
    pub r#sort_columns: Option<Vec<super::super::types::glue::PartitionStorageDescriptorSortColumn>>,
    /// True if the table data is stored in subdirectories, or False if not.
    #[builder(into)]
    #[serde(rename = "storedAsSubDirectories")]
    pub r#stored_as_sub_directories: Option<bool>,
}
