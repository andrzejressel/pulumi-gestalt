#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointS3Settings {
    /// Whether to add column name information to the .csv output file. Default is `false`.
    #[builder(into)]
    #[serde(rename = "addColumnName")]
    pub r#add_column_name: Option<bool>,
    /// S3 object prefix.
    #[builder(into)]
    #[serde(rename = "bucketFolder")]
    pub r#bucket_folder: Option<String>,
    /// S3 bucket name.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Option<String>,
    /// Predefined (canned) access control list for objects created in an S3 bucket. Valid values include `none`, `private`, `public-read`, `public-read-write`, `authenticated-read`, `aws-exec-read`, `bucket-owner-read`, and `bucket-owner-full-control`. Default is `none`.
    #[builder(into)]
    #[serde(rename = "cannedAclForObjects")]
    pub r#canned_acl_for_objects: Option<String>,
    /// Whether to write insert and update operations to .csv or .parquet output files. Default is `false`.
    #[builder(into)]
    #[serde(rename = "cdcInsertsAndUpdates")]
    pub r#cdc_inserts_and_updates: Option<bool>,
    /// Whether to write insert operations to .csv or .parquet output files. Default is `false`.
    #[builder(into)]
    #[serde(rename = "cdcInsertsOnly")]
    pub r#cdc_inserts_only: Option<bool>,
    /// Maximum length of the interval, defined in seconds, after which to output a file to Amazon S3. Default is `60`.
    #[builder(into)]
    #[serde(rename = "cdcMaxBatchInterval")]
    pub r#cdc_max_batch_interval: Option<i32>,
    /// Minimum file size condition as defined in kilobytes to output a file to Amazon S3. Default is `32000`. **NOTE:** Previously, this setting was measured in megabytes but now represents kilobytes. Update configurations accordingly.
    #[builder(into)]
    #[serde(rename = "cdcMinFileSize")]
    pub r#cdc_min_file_size: Option<i32>,
    /// Folder path of CDC files. For an S3 source, this setting is required if a task captures change data; otherwise, it's optional. If `cdc_path` is set, AWS DMS reads CDC files from this path and replicates the data changes to the target endpoint. Supported in AWS DMS versions 3.4.2 and later.
    #[builder(into)]
    #[serde(rename = "cdcPath")]
    pub r#cdc_path: Option<String>,
    /// Set to compress target files. Default is `NONE`. Valid values are `GZIP` and `NONE`.
    #[builder(into)]
    #[serde(rename = "compressionType")]
    pub r#compression_type: Option<String>,
    /// Delimiter used to separate columns in the source files. Default is `,`.
    #[builder(into)]
    #[serde(rename = "csvDelimiter")]
    pub r#csv_delimiter: Option<String>,
    /// String to use for all columns not included in the supplemental log.
    #[builder(into)]
    #[serde(rename = "csvNoSupValue")]
    pub r#csv_no_sup_value: Option<String>,
    /// String to as null when writing to the target.
    #[builder(into)]
    #[serde(rename = "csvNullValue")]
    pub r#csv_null_value: Option<String>,
    /// Delimiter used to separate rows in the source files. Default is `\n`.
    #[builder(into)]
    #[serde(rename = "csvRowDelimiter")]
    pub r#csv_row_delimiter: Option<String>,
    /// Output format for the files that AWS DMS uses to create S3 objects. Valid values are `csv` and `parquet`. Default is `csv`.
    #[builder(into)]
    #[serde(rename = "dataFormat")]
    pub r#data_format: Option<String>,
    /// Size of one data page in bytes. Default is `1048576` (1 MiB).
    #[builder(into)]
    #[serde(rename = "dataPageSize")]
    pub r#data_page_size: Option<i32>,
    /// Date separating delimiter to use during folder partitioning. Valid values are `SLASH`, `UNDERSCORE`, `DASH`, and `NONE`. Default is `SLASH`.
    #[builder(into)]
    #[serde(rename = "datePartitionDelimiter")]
    pub r#date_partition_delimiter: Option<String>,
    /// Partition S3 bucket folders based on transaction commit dates. Default is `false`.
    #[builder(into)]
    #[serde(rename = "datePartitionEnabled")]
    pub r#date_partition_enabled: Option<bool>,
    /// Date format to use during folder partitioning. Use this parameter when `date_partition_enabled` is set to true. Valid values are `YYYYMMDD`, `YYYYMMDDHH`, `YYYYMM`, `MMYYYYDD`, and `DDMMYYYY`. Default is `YYYYMMDD`.
    #[builder(into)]
    #[serde(rename = "datePartitionSequence")]
    pub r#date_partition_sequence: Option<String>,
    /// Maximum size in bytes of an encoded dictionary page of a column. Default is `1048576` (1 MiB).
    #[builder(into)]
    #[serde(rename = "dictPageSizeLimit")]
    pub r#dict_page_size_limit: Option<i32>,
    /// Whether to enable statistics for Parquet pages and row groups. Default is `true`.
    #[builder(into)]
    #[serde(rename = "enableStatistics")]
    pub r#enable_statistics: Option<bool>,
    /// Type of encoding to use. Value values are `rle_dictionary`, `plain`, and `plain_dictionary`. Default is `rle_dictionary`.
    #[builder(into)]
    #[serde(rename = "encodingType")]
    pub r#encoding_type: Option<String>,
    /// Server-side encryption mode that you want to encrypt your .csv or .parquet object files copied to S3. Valid values are `SSE_S3` and `SSE_KMS`. Default is `SSE_S3`.
    #[builder(into)]
    #[serde(rename = "encryptionMode")]
    pub r#encryption_mode: Option<String>,
    /// JSON document that describes how AWS DMS should interpret the data.
    #[builder(into)]
    #[serde(rename = "externalTableDefinition")]
    pub r#external_table_definition: Option<String>,
    /// Whether to integrate AWS Glue Data Catalog with an Amazon S3 target. See [Using AWS Glue Data Catalog with an Amazon S3 target for AWS DMS](https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.GlueCatalog) for more information. Default is `false`.
    #[builder(into)]
    #[serde(rename = "glueCatalogGeneration")]
    pub r#glue_catalog_generation: Option<bool>,
    /// When this value is set to `1`, DMS ignores the first row header in a .csv file. Default is `0`.
    #[builder(into)]
    #[serde(rename = "ignoreHeaderRows")]
    pub r#ignore_header_rows: Option<i32>,
    /// Whether to enable a full load to write INSERT operations to the .csv output files only to indicate how the rows were added to the source database. Default is `false`.
    #[builder(into)]
    #[serde(rename = "includeOpForFullLoad")]
    pub r#include_op_for_full_load: Option<bool>,
    /// Maximum size (in KB) of any .csv file to be created while migrating to an S3 target during full load. Valid values are from `1` to `1048576`. Default is `1048576` (1 GB).
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: Option<i32>,
    /// Specifies the precision of any TIMESTAMP column values written to an S3 object file in .parquet format. Default is `false`.
    #[builder(into)]
    #[serde(rename = "parquetTimestampInMillisecond")]
    pub r#parquet_timestamp_in_millisecond: Option<bool>,
    /// Version of the .parquet file format. Default is `parquet-1-0`. Valid values are `parquet-1-0` and `parquet-2-0`.
    #[builder(into)]
    #[serde(rename = "parquetVersion")]
    pub r#parquet_version: Option<String>,
    /// Whether DMS saves the transaction order for a CDC load on the S3 target specified by `cdc_path`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "preserveTransactions")]
    pub r#preserve_transactions: Option<bool>,
    /// For an S3 source, whether each leading double quotation mark has to be followed by an ending double quotation mark. Default is `true`.
    #[builder(into)]
    #[serde(rename = "rfc4180")]
    pub r#rfc_4180: Option<bool>,
    /// Number of rows in a row group. Default is `10000`.
    #[builder(into)]
    #[serde(rename = "rowGroupLength")]
    pub r#row_group_length: Option<i32>,
    /// ARN or Id of KMS Key to use when `encryption_mode` is `SSE_KMS`.
    #[builder(into)]
    #[serde(rename = "serverSideEncryptionKmsKeyId")]
    pub r#server_side_encryption_kms_key_id: Option<String>,
    /// ARN of the IAM Role with permissions to read from or write to the S3 Bucket.
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Option<String>,
    /// Column to add with timestamp information to the endpoint data for an Amazon S3 target.
    #[builder(into)]
    #[serde(rename = "timestampColumnName")]
    pub r#timestamp_column_name: Option<String>,
    /// Whether to use `csv_no_sup_value` for columns not included in the supplemental log.
    #[builder(into)]
    #[serde(rename = "useCsvNoSupValue")]
    pub r#use_csv_no_sup_value: Option<bool>,
    /// When set to true, uses the task start time as the timestamp column value instead of the time data is written to target. For full load, when set to true, each row of the timestamp column contains the task start time. For CDC loads, each row of the timestamp column contains the transaction commit time. When set to false, the full load timestamp in the timestamp column increments with the time data arrives at the target. Default is `false`.
    #[builder(into)]
    #[serde(rename = "useTaskStartTimeForFullLoadTimestamp")]
    pub r#use_task_start_time_for_full_load_timestamp: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointS3Settings {
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
                    "add_column_name",
                    &self.r#add_column_name,
                ),
                to_pulumi_object_field(
                    "bucket_folder",
                    &self.r#bucket_folder,
                ),
                to_pulumi_object_field(
                    "bucket_name",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "canned_acl_for_objects",
                    &self.r#canned_acl_for_objects,
                ),
                to_pulumi_object_field(
                    "cdc_inserts_and_updates",
                    &self.r#cdc_inserts_and_updates,
                ),
                to_pulumi_object_field(
                    "cdc_inserts_only",
                    &self.r#cdc_inserts_only,
                ),
                to_pulumi_object_field(
                    "cdc_max_batch_interval",
                    &self.r#cdc_max_batch_interval,
                ),
                to_pulumi_object_field(
                    "cdc_min_file_size",
                    &self.r#cdc_min_file_size,
                ),
                to_pulumi_object_field(
                    "cdc_path",
                    &self.r#cdc_path,
                ),
                to_pulumi_object_field(
                    "compression_type",
                    &self.r#compression_type,
                ),
                to_pulumi_object_field(
                    "csv_delimiter",
                    &self.r#csv_delimiter,
                ),
                to_pulumi_object_field(
                    "csv_no_sup_value",
                    &self.r#csv_no_sup_value,
                ),
                to_pulumi_object_field(
                    "csv_null_value",
                    &self.r#csv_null_value,
                ),
                to_pulumi_object_field(
                    "csv_row_delimiter",
                    &self.r#csv_row_delimiter,
                ),
                to_pulumi_object_field(
                    "data_format",
                    &self.r#data_format,
                ),
                to_pulumi_object_field(
                    "data_page_size",
                    &self.r#data_page_size,
                ),
                to_pulumi_object_field(
                    "date_partition_delimiter",
                    &self.r#date_partition_delimiter,
                ),
                to_pulumi_object_field(
                    "date_partition_enabled",
                    &self.r#date_partition_enabled,
                ),
                to_pulumi_object_field(
                    "date_partition_sequence",
                    &self.r#date_partition_sequence,
                ),
                to_pulumi_object_field(
                    "dict_page_size_limit",
                    &self.r#dict_page_size_limit,
                ),
                to_pulumi_object_field(
                    "enable_statistics",
                    &self.r#enable_statistics,
                ),
                to_pulumi_object_field(
                    "encoding_type",
                    &self.r#encoding_type,
                ),
                to_pulumi_object_field(
                    "encryption_mode",
                    &self.r#encryption_mode,
                ),
                to_pulumi_object_field(
                    "external_table_definition",
                    &self.r#external_table_definition,
                ),
                to_pulumi_object_field(
                    "glue_catalog_generation",
                    &self.r#glue_catalog_generation,
                ),
                to_pulumi_object_field(
                    "ignore_header_rows",
                    &self.r#ignore_header_rows,
                ),
                to_pulumi_object_field(
                    "include_op_for_full_load",
                    &self.r#include_op_for_full_load,
                ),
                to_pulumi_object_field(
                    "max_file_size",
                    &self.r#max_file_size,
                ),
                to_pulumi_object_field(
                    "parquet_timestamp_in_millisecond",
                    &self.r#parquet_timestamp_in_millisecond,
                ),
                to_pulumi_object_field(
                    "parquet_version",
                    &self.r#parquet_version,
                ),
                to_pulumi_object_field(
                    "preserve_transactions",
                    &self.r#preserve_transactions,
                ),
                to_pulumi_object_field(
                    "rfc_4180",
                    &self.r#rfc_4180,
                ),
                to_pulumi_object_field(
                    "row_group_length",
                    &self.r#row_group_length,
                ),
                to_pulumi_object_field(
                    "server_side_encryption_kms_key_id",
                    &self.r#server_side_encryption_kms_key_id,
                ),
                to_pulumi_object_field(
                    "service_access_role_arn",
                    &self.r#service_access_role_arn,
                ),
                to_pulumi_object_field(
                    "timestamp_column_name",
                    &self.r#timestamp_column_name,
                ),
                to_pulumi_object_field(
                    "use_csv_no_sup_value",
                    &self.r#use_csv_no_sup_value,
                ),
                to_pulumi_object_field(
                    "use_task_start_time_for_full_load_timestamp",
                    &self.r#use_task_start_time_for_full_load_timestamp,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointS3Settings {
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
                    r#add_column_name: {
                        let field_value = match fields_map.get("add_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'add_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_folder: {
                        let field_value = match fields_map.get("bucket_folder") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_folder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#canned_acl_for_objects: {
                        let field_value = match fields_map.get("canned_acl_for_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'canned_acl_for_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_inserts_and_updates: {
                        let field_value = match fields_map.get("cdc_inserts_and_updates") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdc_inserts_and_updates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_inserts_only: {
                        let field_value = match fields_map.get("cdc_inserts_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdc_inserts_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_max_batch_interval: {
                        let field_value = match fields_map.get("cdc_max_batch_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdc_max_batch_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_min_file_size: {
                        let field_value = match fields_map.get("cdc_min_file_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdc_min_file_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_path: {
                        let field_value = match fields_map.get("cdc_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdc_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression_type: {
                        let field_value = match fields_map.get("compression_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_delimiter: {
                        let field_value = match fields_map.get("csv_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_no_sup_value: {
                        let field_value = match fields_map.get("csv_no_sup_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_no_sup_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_null_value: {
                        let field_value = match fields_map.get("csv_null_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_null_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_row_delimiter: {
                        let field_value = match fields_map.get("csv_row_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_row_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_format: {
                        let field_value = match fields_map.get("data_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_page_size: {
                        let field_value = match fields_map.get("data_page_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_page_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_delimiter: {
                        let field_value = match fields_map.get("date_partition_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_partition_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_enabled: {
                        let field_value = match fields_map.get("date_partition_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_partition_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_sequence: {
                        let field_value = match fields_map.get("date_partition_sequence") {
                            Some(value) => value,
                            None => bail!("Missing field 'date_partition_sequence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dict_page_size_limit: {
                        let field_value = match fields_map.get("dict_page_size_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'dict_page_size_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_statistics: {
                        let field_value = match fields_map.get("enable_statistics") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_statistics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoding_type: {
                        let field_value = match fields_map.get("encoding_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoding_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_mode: {
                        let field_value = match fields_map.get("encryption_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_table_definition: {
                        let field_value = match fields_map.get("external_table_definition") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_table_definition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#glue_catalog_generation: {
                        let field_value = match fields_map.get("glue_catalog_generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'glue_catalog_generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_header_rows: {
                        let field_value = match fields_map.get("ignore_header_rows") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_header_rows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_op_for_full_load: {
                        let field_value = match fields_map.get("include_op_for_full_load") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_op_for_full_load' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_file_size: {
                        let field_value = match fields_map.get("max_file_size") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_file_size' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_timestamp_in_millisecond: {
                        let field_value = match fields_map.get("parquet_timestamp_in_millisecond") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquet_timestamp_in_millisecond' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_version: {
                        let field_value = match fields_map.get("parquet_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquet_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_transactions: {
                        let field_value = match fields_map.get("preserve_transactions") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserve_transactions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rfc_4180: {
                        let field_value = match fields_map.get("rfc_4180") {
                            Some(value) => value,
                            None => bail!("Missing field 'rfc_4180' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_group_length: {
                        let field_value = match fields_map.get("row_group_length") {
                            Some(value) => value,
                            None => bail!("Missing field 'row_group_length' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_encryption_kms_key_id: {
                        let field_value = match fields_map.get("server_side_encryption_kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_side_encryption_kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_role_arn: {
                        let field_value = match fields_map.get("service_access_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_access_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_column_name: {
                        let field_value = match fields_map.get("timestamp_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_csv_no_sup_value: {
                        let field_value = match fields_map.get("use_csv_no_sup_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_csv_no_sup_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_task_start_time_for_full_load_timestamp: {
                        let field_value = match fields_map.get("use_task_start_time_for_full_load_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_task_start_time_for_full_load_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
