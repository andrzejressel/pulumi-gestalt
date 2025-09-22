#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamIcebergConfigurationDestinationTableConfiguration {
    /// The name of the Apache Iceberg database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// The table specific S3 error output prefix. All the errors that occurred while delivering to this table will be prefixed with this value in S3 destination.
    #[builder(into)]
    #[serde(rename = "s3ErrorOutputPrefix")]
    pub r#s_3_error_output_prefix: Option<String>,
    /// The name of the Apache Iceberg Table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// A list of unique keys for a given Apache Iceberg table. Firehose will use these for running Create, Update, or Delete operations on the given Iceberg table.
    #[builder(into)]
    #[serde(rename = "uniqueKeys")]
    pub r#unique_keys: Option<Vec<String>>,
}
