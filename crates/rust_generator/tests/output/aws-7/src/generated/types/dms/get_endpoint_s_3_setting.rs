#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointS3Setting {
    #[builder(into)]
    pub r#add_column_name: bool,
    #[builder(into)]
    pub r#bucket_folder: String,
    #[builder(into)]
    pub r#bucket_name: String,
    #[builder(into)]
    pub r#canned_acl_for_objects: String,
    #[builder(into)]
    pub r#cdc_inserts_and_updates: bool,
    #[builder(into)]
    pub r#cdc_inserts_only: bool,
    #[builder(into)]
    pub r#cdc_max_batch_interval: i32,
    #[builder(into)]
    pub r#cdc_min_file_size: i32,
    #[builder(into)]
    pub r#cdc_path: String,
    #[builder(into)]
    pub r#compression_type: String,
    #[builder(into)]
    pub r#csv_delimiter: String,
    #[builder(into)]
    pub r#csv_no_sup_value: String,
    #[builder(into)]
    pub r#csv_null_value: String,
    #[builder(into)]
    pub r#csv_row_delimiter: String,
    #[builder(into)]
    pub r#data_format: String,
    #[builder(into)]
    pub r#data_page_size: i32,
    #[builder(into)]
    pub r#date_partition_delimiter: String,
    #[builder(into)]
    pub r#date_partition_enabled: bool,
    #[builder(into)]
    pub r#date_partition_sequence: String,
    #[builder(into)]
    pub r#dict_page_size_limit: i32,
    #[builder(into)]
    pub r#enable_statistics: bool,
    #[builder(into)]
    pub r#encoding_type: String,
    #[builder(into)]
    pub r#encryption_mode: String,
    #[builder(into)]
    pub r#external_table_definition: String,
    #[builder(into)]
    pub r#glue_catalog_generation: bool,
    #[builder(into)]
    pub r#ignore_header_rows: i32,
    #[builder(into)]
    pub r#ignore_headers_row: i32,
    #[builder(into)]
    pub r#include_op_for_full_load: bool,
    #[builder(into)]
    pub r#max_file_size: i32,
    #[builder(into)]
    pub r#parquet_timestamp_in_millisecond: bool,
    #[builder(into)]
    pub r#parquet_version: String,
    #[builder(into)]
    pub r#preserve_transactions: bool,
    #[builder(into)]
    pub r#rfc_4180: bool,
    #[builder(into)]
    pub r#row_group_length: i32,
    #[builder(into)]
    pub r#server_side_encryption_kms_key_id: String,
    #[builder(into)]
    pub r#service_access_role_arn: String,
    #[builder(into)]
    pub r#timestamp_column_name: String,
    #[builder(into)]
    pub r#use_csv_no_sup_value: bool,
    #[builder(into)]
    pub r#use_task_start_time_for_full_load_timestamp: bool,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetEndpointS3Setting {
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
                    "addColumnName",
                    &self.r#add_column_name,
                ),
                to_pulumi_object_field(
                    "bucketFolder",
                    &self.r#bucket_folder,
                ),
                to_pulumi_object_field(
                    "bucketName",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "cannedAclForObjects",
                    &self.r#canned_acl_for_objects,
                ),
                to_pulumi_object_field(
                    "cdcInsertsAndUpdates",
                    &self.r#cdc_inserts_and_updates,
                ),
                to_pulumi_object_field(
                    "cdcInsertsOnly",
                    &self.r#cdc_inserts_only,
                ),
                to_pulumi_object_field(
                    "cdcMaxBatchInterval",
                    &self.r#cdc_max_batch_interval,
                ),
                to_pulumi_object_field(
                    "cdcMinFileSize",
                    &self.r#cdc_min_file_size,
                ),
                to_pulumi_object_field(
                    "cdcPath",
                    &self.r#cdc_path,
                ),
                to_pulumi_object_field(
                    "compressionType",
                    &self.r#compression_type,
                ),
                to_pulumi_object_field(
                    "csvDelimiter",
                    &self.r#csv_delimiter,
                ),
                to_pulumi_object_field(
                    "csvNoSupValue",
                    &self.r#csv_no_sup_value,
                ),
                to_pulumi_object_field(
                    "csvNullValue",
                    &self.r#csv_null_value,
                ),
                to_pulumi_object_field(
                    "csvRowDelimiter",
                    &self.r#csv_row_delimiter,
                ),
                to_pulumi_object_field(
                    "dataFormat",
                    &self.r#data_format,
                ),
                to_pulumi_object_field(
                    "dataPageSize",
                    &self.r#data_page_size,
                ),
                to_pulumi_object_field(
                    "datePartitionDelimiter",
                    &self.r#date_partition_delimiter,
                ),
                to_pulumi_object_field(
                    "datePartitionEnabled",
                    &self.r#date_partition_enabled,
                ),
                to_pulumi_object_field(
                    "datePartitionSequence",
                    &self.r#date_partition_sequence,
                ),
                to_pulumi_object_field(
                    "dictPageSizeLimit",
                    &self.r#dict_page_size_limit,
                ),
                to_pulumi_object_field(
                    "enableStatistics",
                    &self.r#enable_statistics,
                ),
                to_pulumi_object_field(
                    "encodingType",
                    &self.r#encoding_type,
                ),
                to_pulumi_object_field(
                    "encryptionMode",
                    &self.r#encryption_mode,
                ),
                to_pulumi_object_field(
                    "externalTableDefinition",
                    &self.r#external_table_definition,
                ),
                to_pulumi_object_field(
                    "glueCatalogGeneration",
                    &self.r#glue_catalog_generation,
                ),
                to_pulumi_object_field(
                    "ignoreHeaderRows",
                    &self.r#ignore_header_rows,
                ),
                to_pulumi_object_field(
                    "ignoreHeadersRow",
                    &self.r#ignore_headers_row,
                ),
                to_pulumi_object_field(
                    "includeOpForFullLoad",
                    &self.r#include_op_for_full_load,
                ),
                to_pulumi_object_field(
                    "maxFileSize",
                    &self.r#max_file_size,
                ),
                to_pulumi_object_field(
                    "parquetTimestampInMillisecond",
                    &self.r#parquet_timestamp_in_millisecond,
                ),
                to_pulumi_object_field(
                    "parquetVersion",
                    &self.r#parquet_version,
                ),
                to_pulumi_object_field(
                    "preserveTransactions",
                    &self.r#preserve_transactions,
                ),
                to_pulumi_object_field(
                    "rfc4180",
                    &self.r#rfc_4180,
                ),
                to_pulumi_object_field(
                    "rowGroupLength",
                    &self.r#row_group_length,
                ),
                to_pulumi_object_field(
                    "serverSideEncryptionKmsKeyId",
                    &self.r#server_side_encryption_kms_key_id,
                ),
                to_pulumi_object_field(
                    "serviceAccessRoleArn",
                    &self.r#service_access_role_arn,
                ),
                to_pulumi_object_field(
                    "timestampColumnName",
                    &self.r#timestamp_column_name,
                ),
                to_pulumi_object_field(
                    "useCsvNoSupValue",
                    &self.r#use_csv_no_sup_value,
                ),
                to_pulumi_object_field(
                    "useTaskStartTimeForFullLoadTimestamp",
                    &self.r#use_task_start_time_for_full_load_timestamp,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetEndpointS3Setting {
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
                        let field_value = match fields_map.get("addColumnName") {
                            Some(value) => value,
                            None => bail!("Missing field 'addColumnName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_folder: {
                        let field_value = match fields_map.get("bucketFolder") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketFolder' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucketName") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#canned_acl_for_objects: {
                        let field_value = match fields_map.get("cannedAclForObjects") {
                            Some(value) => value,
                            None => bail!("Missing field 'cannedAclForObjects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_inserts_and_updates: {
                        let field_value = match fields_map.get("cdcInsertsAndUpdates") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdcInsertsAndUpdates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_inserts_only: {
                        let field_value = match fields_map.get("cdcInsertsOnly") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdcInsertsOnly' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_max_batch_interval: {
                        let field_value = match fields_map.get("cdcMaxBatchInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdcMaxBatchInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_min_file_size: {
                        let field_value = match fields_map.get("cdcMinFileSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdcMinFileSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cdc_path: {
                        let field_value = match fields_map.get("cdcPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'cdcPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression_type: {
                        let field_value = match fields_map.get("compressionType") {
                            Some(value) => value,
                            None => bail!("Missing field 'compressionType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_delimiter: {
                        let field_value = match fields_map.get("csvDelimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'csvDelimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_no_sup_value: {
                        let field_value = match fields_map.get("csvNoSupValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'csvNoSupValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_null_value: {
                        let field_value = match fields_map.get("csvNullValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'csvNullValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_row_delimiter: {
                        let field_value = match fields_map.get("csvRowDelimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'csvRowDelimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_format: {
                        let field_value = match fields_map.get("dataFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_page_size: {
                        let field_value = match fields_map.get("dataPageSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'dataPageSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_delimiter: {
                        let field_value = match fields_map.get("datePartitionDelimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'datePartitionDelimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_enabled: {
                        let field_value = match fields_map.get("datePartitionEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'datePartitionEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#date_partition_sequence: {
                        let field_value = match fields_map.get("datePartitionSequence") {
                            Some(value) => value,
                            None => bail!("Missing field 'datePartitionSequence' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dict_page_size_limit: {
                        let field_value = match fields_map.get("dictPageSizeLimit") {
                            Some(value) => value,
                            None => bail!("Missing field 'dictPageSizeLimit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_statistics: {
                        let field_value = match fields_map.get("enableStatistics") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableStatistics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoding_type: {
                        let field_value = match fields_map.get("encodingType") {
                            Some(value) => value,
                            None => bail!("Missing field 'encodingType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_mode: {
                        let field_value = match fields_map.get("encryptionMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_table_definition: {
                        let field_value = match fields_map.get("externalTableDefinition") {
                            Some(value) => value,
                            None => bail!("Missing field 'externalTableDefinition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#glue_catalog_generation: {
                        let field_value = match fields_map.get("glueCatalogGeneration") {
                            Some(value) => value,
                            None => bail!("Missing field 'glueCatalogGeneration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_header_rows: {
                        let field_value = match fields_map.get("ignoreHeaderRows") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreHeaderRows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_headers_row: {
                        let field_value = match fields_map.get("ignoreHeadersRow") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreHeadersRow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_op_for_full_load: {
                        let field_value = match fields_map.get("includeOpForFullLoad") {
                            Some(value) => value,
                            None => bail!("Missing field 'includeOpForFullLoad' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_file_size: {
                        let field_value = match fields_map.get("maxFileSize") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxFileSize' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_timestamp_in_millisecond: {
                        let field_value = match fields_map.get("parquetTimestampInMillisecond") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquetTimestampInMillisecond' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_version: {
                        let field_value = match fields_map.get("parquetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preserve_transactions: {
                        let field_value = match fields_map.get("preserveTransactions") {
                            Some(value) => value,
                            None => bail!("Missing field 'preserveTransactions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rfc_4180: {
                        let field_value = match fields_map.get("rfc4180") {
                            Some(value) => value,
                            None => bail!("Missing field 'rfc4180' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_group_length: {
                        let field_value = match fields_map.get("rowGroupLength") {
                            Some(value) => value,
                            None => bail!("Missing field 'rowGroupLength' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_side_encryption_kms_key_id: {
                        let field_value = match fields_map.get("serverSideEncryptionKmsKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverSideEncryptionKmsKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_access_role_arn: {
                        let field_value = match fields_map.get("serviceAccessRoleArn") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccessRoleArn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_column_name: {
                        let field_value = match fields_map.get("timestampColumnName") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampColumnName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_csv_no_sup_value: {
                        let field_value = match fields_map.get("useCsvNoSupValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'useCsvNoSupValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_task_start_time_for_full_load_timestamp: {
                        let field_value = match fields_map.get("useTaskStartTimeForFullLoadTimestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'useTaskStartTimeForFullLoadTimestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
