#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobLoad {
    /// Accept rows that are missing trailing optional columns. The missing values are treated as nulls.
    /// If false, records with missing trailing columns are treated as bad records, and if there are too many bad records,
    /// an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[builder(into)]
    pub r#allow_jagged_rows: Option<bool>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file.
    /// The default value is false.
    #[builder(into)]
    pub r#allow_quoted_newlines: Option<bool>,
    /// Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    #[builder(into)]
    pub r#autodetect: Option<bool>,
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into)]
    pub r#create_disposition: Option<String>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_encryption_configuration: Option<Box<super::super::types::bigquery::JobLoadDestinationEncryptionConfiguration>>,
    /// The destination table to load the data into.
    /// Structure is documented below.
    #[builder(into)]
    pub r#destination_table: Box<super::super::types::bigquery::JobLoadDestinationTable>,
    /// The character encoding of the data. The supported values are UTF-8 or ISO-8859-1.
    /// The default value is UTF-8. BigQuery decodes the data after the raw, binary data
    /// has been split using the values of the quote and fieldDelimiter properties.
    #[builder(into)]
    pub r#encoding: Option<String>,
    /// The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character.
    /// To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts
    /// the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the
    /// data in its raw, binary state. BigQuery also supports the escape sequence "\t" to specify a tab separator.
    /// The default value is a comma (',').
    #[builder(into)]
    pub r#field_delimiter: Option<String>,
    /// Indicates if BigQuery should allow extra values that are not represented in the table schema.
    /// If true, the extra values are ignored. If false, records with extra columns are treated as bad records,
    /// and if there are too many bad records, an invalid error is returned in the job result.
    /// The default value is false. The sourceFormat property determines what BigQuery treats as an extra value:
    /// CSV: Trailing columns
    /// JSON: Named values that don't match any column names
    #[builder(into)]
    pub r#ignore_unknown_values: Option<bool>,
    /// If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON.
    /// For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited
    /// GeoJSON: set to GEOJSON.
    #[builder(into)]
    pub r#json_extension: Option<String>,
    /// The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value,
    /// an invalid error is returned in the job result. The default value is 0, which requires that all records are valid.
    #[builder(into)]
    pub r#max_bad_records: Option<i32>,
    /// Specifies a string that represents a null value in a CSV file. The default value is the empty string. If you set this
    /// property to a custom value, BigQuery throws an error if an
    /// empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as
    /// an empty value.
    #[builder(into)]
    pub r#null_marker: Option<String>,
    /// Parquet Options for load and make external tables.
    /// Structure is documented below.
    #[builder(into)]
    pub r#parquet_options: Option<Box<super::super::types::bigquery::JobLoadParquetOptions>>,
    /// If sourceFormat is set to "DATASTORE_BACKUP", indicates which entity properties to load into BigQuery from a Cloud Datastore backup.
    /// Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties.
    /// If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[builder(into)]
    pub r#projection_fields: Option<Vec<String>>,
    /// The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding,
    /// and then uses the first byte of the encoded string to split the data in its raw, binary state.
    /// The default value is a double-quote ('"'). If your data does not contain quoted sections, set the property value to an empty string.
    /// If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    #[builder(into)]
    pub r#quote: Option<String>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or
    /// supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND;
    /// when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators.
    /// For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified:
    /// ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema.
    /// ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[builder(into)]
    pub r#schema_update_options: Option<Vec<String>>,
    /// The number of rows at the top of a CSV file that BigQuery will skip when loading the data.
    /// The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    /// When autodetect is on, the behavior is the following:
    /// skipLeadingRows unspecified - Autodetect tries to detect headers in the first row. If they are not detected,
    /// the row is read as data. Otherwise data is read starting from the second row.
    /// skipLeadingRows is 0 - Instructs autodetect that there are no headers and data should be read starting from the first row.
    /// skipLeadingRows = N > 0 - Autodetect skips N-1 rows and tries to detect headers in row N. If headers are not detected,
    /// row N is just skipped. Otherwise row N is used to extract column names for the detected schema.
    #[builder(into)]
    pub r#skip_leading_rows: Option<i32>,
    /// The format of the data files. For CSV files, specify "CSV". For datastore backups, specify "DATASTORE_BACKUP".
    /// For newline-delimited JSON, specify "NEWLINE_DELIMITED_JSON". For Avro, specify "AVRO". For parquet, specify "PARQUET".
    /// For orc, specify "ORC". [Beta] For Bigtable, specify "BIGTABLE".
    /// The default value is CSV.
    #[builder(into)]
    pub r#source_format: Option<String>,
    /// The fully-qualified URIs that point to your data in Google Cloud.
    /// For Google Cloud Storage URIs: Each URI can contain one '\*' wildcard character
    /// and it must come after the 'bucket' name. Size limits related to load jobs apply
    /// to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be
    /// specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table.
    /// For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '\*' wildcard character is not allowed.
    #[builder(into)]
    pub r#source_uris: Vec<String>,
    /// Time-based partitioning specification for the destination table.
    /// Structure is documented below.
    #[builder(into)]
    pub r#time_partitioning: Option<Box<super::super::types::bigquery::JobLoadTimePartitioning>>,
    /// Specifies the action that occurs if the destination table already exists. The following values are supported:
    /// WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.
    /// WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.
    /// Each action is atomic and only occurs if BigQuery is able to complete the job successfully.
    /// Creation, truncation and append actions occur as one atomic update upon job completion.
    /// Default value is `WRITE_EMPTY`.
    /// Possible values are: `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
    #[builder(into)]
    pub r#write_disposition: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobLoad {
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
                    "allowJaggedRows",
                    &self.r#allow_jagged_rows,
                ),
                to_pulumi_object_field(
                    "allowQuotedNewlines",
                    &self.r#allow_quoted_newlines,
                ),
                to_pulumi_object_field(
                    "autodetect",
                    &self.r#autodetect,
                ),
                to_pulumi_object_field(
                    "createDisposition",
                    &self.r#create_disposition,
                ),
                to_pulumi_object_field(
                    "destinationEncryptionConfiguration",
                    &self.r#destination_encryption_configuration,
                ),
                to_pulumi_object_field(
                    "destinationTable",
                    &self.r#destination_table,
                ),
                to_pulumi_object_field(
                    "encoding",
                    &self.r#encoding,
                ),
                to_pulumi_object_field(
                    "fieldDelimiter",
                    &self.r#field_delimiter,
                ),
                to_pulumi_object_field(
                    "ignoreUnknownValues",
                    &self.r#ignore_unknown_values,
                ),
                to_pulumi_object_field(
                    "jsonExtension",
                    &self.r#json_extension,
                ),
                to_pulumi_object_field(
                    "maxBadRecords",
                    &self.r#max_bad_records,
                ),
                to_pulumi_object_field(
                    "nullMarker",
                    &self.r#null_marker,
                ),
                to_pulumi_object_field(
                    "parquetOptions",
                    &self.r#parquet_options,
                ),
                to_pulumi_object_field(
                    "projectionFields",
                    &self.r#projection_fields,
                ),
                to_pulumi_object_field(
                    "quote",
                    &self.r#quote,
                ),
                to_pulumi_object_field(
                    "schemaUpdateOptions",
                    &self.r#schema_update_options,
                ),
                to_pulumi_object_field(
                    "skipLeadingRows",
                    &self.r#skip_leading_rows,
                ),
                to_pulumi_object_field(
                    "sourceFormat",
                    &self.r#source_format,
                ),
                to_pulumi_object_field(
                    "sourceUris",
                    &self.r#source_uris,
                ),
                to_pulumi_object_field(
                    "timePartitioning",
                    &self.r#time_partitioning,
                ),
                to_pulumi_object_field(
                    "writeDisposition",
                    &self.r#write_disposition,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobLoad {
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
                    r#allow_jagged_rows: {
                        let field_value = match fields_map.get("allowJaggedRows") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowJaggedRows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_quoted_newlines: {
                        let field_value = match fields_map.get("allowQuotedNewlines") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowQuotedNewlines' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#autodetect: {
                        let field_value = match fields_map.get("autodetect") {
                            Some(value) => value,
                            None => bail!("Missing field 'autodetect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_disposition: {
                        let field_value = match fields_map.get("createDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'createDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_encryption_configuration: {
                        let field_value = match fields_map.get("destinationEncryptionConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationEncryptionConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_table: {
                        let field_value = match fields_map.get("destinationTable") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationTable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encoding: {
                        let field_value = match fields_map.get("encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_delimiter: {
                        let field_value = match fields_map.get("fieldDelimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'fieldDelimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_unknown_values: {
                        let field_value = match fields_map.get("ignoreUnknownValues") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignoreUnknownValues' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_extension: {
                        let field_value = match fields_map.get("jsonExtension") {
                            Some(value) => value,
                            None => bail!("Missing field 'jsonExtension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_bad_records: {
                        let field_value = match fields_map.get("maxBadRecords") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxBadRecords' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#null_marker: {
                        let field_value = match fields_map.get("nullMarker") {
                            Some(value) => value,
                            None => bail!("Missing field 'nullMarker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_options: {
                        let field_value = match fields_map.get("parquetOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquetOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#projection_fields: {
                        let field_value = match fields_map.get("projectionFields") {
                            Some(value) => value,
                            None => bail!("Missing field 'projectionFields' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quote: {
                        let field_value = match fields_map.get("quote") {
                            Some(value) => value,
                            None => bail!("Missing field 'quote' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema_update_options: {
                        let field_value = match fields_map.get("schemaUpdateOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'schemaUpdateOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#skip_leading_rows: {
                        let field_value = match fields_map.get("skipLeadingRows") {
                            Some(value) => value,
                            None => bail!("Missing field 'skipLeadingRows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_format: {
                        let field_value = match fields_map.get("sourceFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_uris: {
                        let field_value = match fields_map.get("sourceUris") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceUris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#time_partitioning: {
                        let field_value = match fields_map.get("timePartitioning") {
                            Some(value) => value,
                            None => bail!("Missing field 'timePartitioning' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#write_disposition: {
                        let field_value = match fields_map.get("writeDisposition") {
                            Some(value) => value,
                            None => bail!("Missing field 'writeDisposition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
