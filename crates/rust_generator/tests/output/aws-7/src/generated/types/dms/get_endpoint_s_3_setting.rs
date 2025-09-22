#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointS3Setting {
    #[builder(into)]
    #[serde(rename = "addColumnName")]
    pub r#add_column_name: bool,
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: String,
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    #[builder(into)]
    #[serde(rename = "cannedAclForObjects")]
    pub r#canned_acl_for_objects: String,
    #[builder(into)]
    #[serde(rename = "cdcInsertsAndUpdates")]
    pub r#cdc_inserts_and_updates: bool,
    #[builder(into)]
    #[serde(rename = "cdcInsertsOnly")]
    pub r#cdc_inserts_only: bool,
    #[builder(into)]
    #[serde(rename = "cdcMaxBatchInterval")]
    pub r#cdc_max_batch_interval: i32,
    #[builder(into)]
    #[serde(rename = "cdcMinFileSize")]
    pub r#cdc_min_file_size: i32,
    #[builder(into)]
    #[serde(rename = "cdcPath")]
    pub r#cdc_path: String,
    #[builder(into)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: String,
    #[builder(into)]
    #[serde(rename = "csvDelimiter")]
    pub r#csv_delimiter: String,
    #[builder(into)]
    #[serde(rename = "csvNoSupValue")]
    pub r#csv_no_sup_value: String,
    #[builder(into)]
    #[serde(rename = "csvNullValue")]
    pub r#csv_null_value: String,
    #[builder(into)]
    #[serde(rename = "csvRowDelimiter")]
    pub r#csv_row_delimiter: String,
    #[builder(into)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: String,
    #[builder(into)]
    #[serde(rename = "dataPageSize")]
    pub r#data_page_size: i32,
    #[builder(into)]
    #[serde(rename = "datePartitionDelimiter")]
    pub r#date_partition_delimiter: String,
    #[builder(into)]
    #[serde(rename = "datePartitionEnabled")]
    pub r#date_partition_enabled: bool,
    #[builder(into)]
    #[serde(rename = "datePartitionSequence")]
    pub r#date_partition_sequence: String,
    #[builder(into)]
    #[serde(rename = "dictPageSizeLimit")]
    pub r#dict_page_size_limit: i32,
    #[builder(into)]
    #[serde(rename = "enableStatistics")]
    pub r#enable_statistics: bool,
    #[builder(into)]
    #[serde(rename = "encodingType")]
    pub r#encoding_type: String,
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: String,
    #[builder(into)]
    #[serde(rename = "externalTableDefinition")]
    pub r#external_table_definition: String,
    #[builder(into)]
    #[serde(rename = "glueCatalogGeneration")]
    pub r#glue_catalog_generation: bool,
    #[builder(into)]
    #[serde(rename = "ignoreHeaderRows")]
    pub r#ignore_header_rows: i32,
    #[builder(into)]
    #[serde(rename = "ignoreHeadersRow")]
    pub r#ignore_headers_row: i32,
    #[builder(into)]
    #[serde(rename = "includeOpForFullLoad")]
    pub r#include_op_for_full_load: bool,
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: i32,
    #[builder(into)]
    #[serde(rename = "parquetTimestampInMillisecond")]
    pub r#parquet_timestamp_in_millisecond: bool,
    #[builder(into)]
    #[serde(rename = "parquetVersion")]
    pub r#parquet_version: String,
    #[builder(into)]
    #[serde(rename = "preserveTransactions")]
    pub r#preserve_transactions: bool,
    #[builder(into)]
    #[serde(rename = "rfc4180")]
    pub r#rfc_4180: bool,
    #[builder(into)]
    #[serde(rename = "rowGroupLength")]
    pub r#row_group_length: i32,
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: String,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
    #[builder(into)]
    #[serde(rename = "timestampColumnName")]
    pub r#timestamp_column_name: String,
    #[builder(into)]
    #[serde(rename = "useCsvNoSupValue")]
    pub r#use_csv_no_sup_value: bool,
    #[builder(into)]
    #[serde(rename = "useTaskStartTimeForFullLoadTimestamp")]
    pub r#use_task_start_time_for_full_load_timestamp: bool,
}
