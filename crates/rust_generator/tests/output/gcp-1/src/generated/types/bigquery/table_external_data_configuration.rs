#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableExternalDataConfiguration {
    /// Let BigQuery try to autodetect the schema
    /// and format of the table.
    #[builder(into)]
    #[serde(rename = "autodetect")]
    pub r#autodetect: bool,
    /// Additional options if `source_format` is set to
    /// "AVRO".  Structure is documented below.
    #[builder(into)]
    #[serde(rename = "avroOptions")]
    pub r#avro_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationAvroOptions>>,
    /// Additional properties to set if
    /// `source_format` is set to "BIGTABLE". Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigtableOptions")]
    pub r#bigtable_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationBigtableOptions>>,
    /// The compression type of the data source.
    /// Valid values are "NONE" or "GZIP".
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Option<String>,
    /// The connection specifying the credentials to be used to read
    /// external storage, such as Azure Blob, Cloud Storage, or S3. The `connection_id` can have
    /// the form `{{project}}.{{location}}.{{connection_id}}`
    /// or `projects/{{project}}/locations/{{location}}/connections/{{connection_id}}`.
    /// 
    /// ~>**NOTE:** If you set `external_data_configuration.connection_id`, the
    /// table schema must be specified using the top-level `schema` field
    /// documented above.
    #[builder(into)]
    #[serde(rename = "connectionId")]
    pub r#connection_id: Option<String>,
    /// Additional properties to set if
    /// `source_format` is set to "CSV". Structure is documented below.
    #[builder(into)]
    #[serde(rename = "csvOptions")]
    pub r#csv_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationCsvOptions>>,
    /// Specifies how source URIs are interpreted for constructing the file set to load.
    /// By default source URIs are expanded against the underlying storage.
    /// Other options include specifying manifest files. Only applicable to object storage systems. Docs
    #[builder(into)]
    #[serde(rename = "fileSetSpecType")]
    pub r#file_set_spec_type: Option<String>,
    /// Additional options if
    /// `source_format` is set to "GOOGLE_SHEETS". Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "googleSheetsOptions")]
    pub r#google_sheets_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationGoogleSheetsOptions>>,
    /// When set, configures hive partitioning
    /// support. Not all storage formats support hive partitioning -- requesting hive
    /// partitioning on an unsupported format will lead to an error, as will providing
    /// an invalid specification. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hivePartitioningOptions")]
    pub r#hive_partitioning_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationHivePartitioningOptions>>,
    /// Indicates if BigQuery should
    /// allow extra values that are not represented in the table schema.
    /// If true, the extra values are ignored. If false, records with
    /// extra columns are treated as bad records, and if there are too
    /// many bad records, an invalid error is returned in the job result.
    /// The default value is false.
    #[builder(into)]
    #[serde(rename = "ignoreUnknownValues")]
    pub r#ignore_unknown_values: Option<bool>,
    /// Used to indicate that a JSON variant, rather than normal JSON, is being used as the sourceFormat. This should only be used in combination with the `JSON` source format. Valid values are: `GEOJSON`.
    #[builder(into)]
    #[serde(rename = "jsonExtension")]
    pub r#json_extension: Option<String>,
    /// Additional properties to set if
    /// `source_format` is set to "JSON". Structure is documented below.
    #[builder(into)]
    #[serde(rename = "jsonOptions")]
    pub r#json_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationJsonOptions>>,
    /// The maximum number of bad records that
    /// BigQuery can ignore when reading data.
    #[builder(into)]
    #[serde(rename = "maxBadRecords")]
    pub r#max_bad_records: Option<i32>,
    /// Metadata Cache Mode for the table. Set this to enable caching of metadata from external data source. Valid values are `AUTOMATIC` and `MANUAL`.
    #[builder(into)]
    #[serde(rename = "metadataCacheMode")]
    pub r#metadata_cache_mode: Option<String>,
    /// Object Metadata is used to create Object Tables. Object Tables contain a listing of objects (with their metadata) found at the sourceUris. If `object_metadata` is set, `source_format` should be omitted.
    #[builder(into)]
    #[serde(rename = "objectMetadata")]
    pub r#object_metadata: Option<String>,
    /// Additional properties to set if
    /// `source_format` is set to "PARQUET". Structure is documented below.
    #[builder(into)]
    #[serde(rename = "parquetOptions")]
    pub r#parquet_options: Option<Box<super::super::types::bigquery::TableExternalDataConfigurationParquetOptions>>,
    /// When creating an external table, the user can provide a reference file with the table schema. This is enabled for the following formats: AVRO, PARQUET, ORC.
    #[builder(into)]
    #[serde(rename = "referenceFileSchemaUri")]
    pub r#reference_file_schema_uri: Option<String>,
    /// A JSON schema for the external table. Schema is required
    /// for CSV and JSON formats if autodetect is not on. Schema is disallowed
    /// for Google Cloud Bigtable, Cloud Datastore backups, Avro, Iceberg, ORC and Parquet formats.
    /// ~>**NOTE:** Because this field expects a JSON string, any changes to the
    /// string will create a diff, even if the JSON itself hasn't changed.
    /// Furthermore drift for this field cannot not be detected because BigQuery
    /// only uses this schema to compute the effective schema for the table, therefore
    /// any changes on the configured value will force the table to be recreated.
    /// This schema is effectively only applied when creating a table from an external
    /// datasource, after creation the computed schema will be stored in
    /// `google_bigquery_table.schema`
    /// 
    /// ~>**NOTE:** If you set `external_data_configuration.connection_id`, the
    /// table schema must be specified using the top-level `schema` field
    /// documented above.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
    /// The data format. Please see sourceFormat under
    /// [ExternalDataConfiguration](https://cloud.google.com/bigquery/docs/reference/rest/v2/tables#externaldataconfiguration)
    /// in Bigquery's public API documentation for supported formats. To use "GOOGLE_SHEETS"
    /// the `scopes` must include "https://www.googleapis.com/auth/drive.readonly".
    #[builder(into)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Option<String>,
    /// A list of the fully-qualified URIs that point to
    /// your data in Google Cloud.
    #[builder(into)]
    #[serde(rename = "sourceUris")]
    pub r#source_uris: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableExternalDataConfiguration {
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
                    "autodetect",
                    &self.r#autodetect,
                ),
                to_pulumi_object_field(
                    "avro_options",
                    &self.r#avro_options,
                ),
                to_pulumi_object_field(
                    "bigtable_options",
                    &self.r#bigtable_options,
                ),
                to_pulumi_object_field(
                    "compression",
                    &self.r#compression,
                ),
                to_pulumi_object_field(
                    "connection_id",
                    &self.r#connection_id,
                ),
                to_pulumi_object_field(
                    "csv_options",
                    &self.r#csv_options,
                ),
                to_pulumi_object_field(
                    "file_set_spec_type",
                    &self.r#file_set_spec_type,
                ),
                to_pulumi_object_field(
                    "google_sheets_options",
                    &self.r#google_sheets_options,
                ),
                to_pulumi_object_field(
                    "hive_partitioning_options",
                    &self.r#hive_partitioning_options,
                ),
                to_pulumi_object_field(
                    "ignore_unknown_values",
                    &self.r#ignore_unknown_values,
                ),
                to_pulumi_object_field(
                    "json_extension",
                    &self.r#json_extension,
                ),
                to_pulumi_object_field(
                    "json_options",
                    &self.r#json_options,
                ),
                to_pulumi_object_field(
                    "max_bad_records",
                    &self.r#max_bad_records,
                ),
                to_pulumi_object_field(
                    "metadata_cache_mode",
                    &self.r#metadata_cache_mode,
                ),
                to_pulumi_object_field(
                    "object_metadata",
                    &self.r#object_metadata,
                ),
                to_pulumi_object_field(
                    "parquet_options",
                    &self.r#parquet_options,
                ),
                to_pulumi_object_field(
                    "reference_file_schema_uri",
                    &self.r#reference_file_schema_uri,
                ),
                to_pulumi_object_field(
                    "schema",
                    &self.r#schema,
                ),
                to_pulumi_object_field(
                    "source_format",
                    &self.r#source_format,
                ),
                to_pulumi_object_field(
                    "source_uris",
                    &self.r#source_uris,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableExternalDataConfiguration {
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
                    r#autodetect: {
                        let field_value = match fields_map.get("autodetect") {
                            Some(value) => value,
                            None => bail!("Missing field 'autodetect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#avro_options: {
                        let field_value = match fields_map.get("avro_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'avro_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bigtable_options: {
                        let field_value = match fields_map.get("bigtable_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'bigtable_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression: {
                        let field_value = match fields_map.get("compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_id: {
                        let field_value = match fields_map.get("connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#csv_options: {
                        let field_value = match fields_map.get("csv_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'csv_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_set_spec_type: {
                        let field_value = match fields_map.get("file_set_spec_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_set_spec_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#google_sheets_options: {
                        let field_value = match fields_map.get("google_sheets_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'google_sheets_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hive_partitioning_options: {
                        let field_value = match fields_map.get("hive_partitioning_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'hive_partitioning_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ignore_unknown_values: {
                        let field_value = match fields_map.get("ignore_unknown_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'ignore_unknown_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_extension: {
                        let field_value = match fields_map.get("json_extension") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_extension' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#json_options: {
                        let field_value = match fields_map.get("json_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'json_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_bad_records: {
                        let field_value = match fields_map.get("max_bad_records") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_bad_records' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_cache_mode: {
                        let field_value = match fields_map.get("metadata_cache_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_cache_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_metadata: {
                        let field_value = match fields_map.get("object_metadata") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_metadata' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parquet_options: {
                        let field_value = match fields_map.get("parquet_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'parquet_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_file_schema_uri: {
                        let field_value = match fields_map.get("reference_file_schema_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_file_schema_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_format: {
                        let field_value = match fields_map.get("source_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_uris: {
                        let field_value = match fields_map.get("source_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
