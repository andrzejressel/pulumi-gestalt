#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobExtract {
    /// The compression type to use for exported files. Possible values include GZIP, DEFLATE, SNAPPY, and NONE.
    /// The default value is NONE. DEFLATE and SNAPPY are only supported for Avro.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Option<String>,
    /// The exported file format. Possible values include CSV, NEWLINE_DELIMITED_JSON and AVRO for tables and SAVED_MODEL for models.
    /// The default value for tables is CSV. Tables with nested or repeated fields cannot be exported as CSV.
    /// The default value for models is SAVED_MODEL.
    #[builder(into)]
    #[serde(rename = "destinationFormat")]
    pub r#destination_format: Option<String>,
    /// A list of fully-qualified Google Cloud Storage URIs where the extracted table should be written.
    #[builder(into)]
    #[serde(rename = "destinationUris")]
    pub r#destination_uris: Vec<String>,
    /// When extracting data in CSV format, this defines the delimiter to use between fields in the exported data.
    /// Default is ','
    #[builder(into)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Option<String>,
    /// Whether to print out a header row in the results. Default is true.
    #[builder(into)]
    #[serde(rename = "printHeader")]
    pub r#print_header: Option<bool>,
    /// A reference to the model being exported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sourceModel")]
    pub r#source_model: Option<Box<super::super::types::bigquery::JobExtractSourceModel>>,
    /// A reference to the table being exported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sourceTable")]
    pub r#source_table: Option<Box<super::super::types::bigquery::JobExtractSourceTable>>,
    /// Whether to use logical types when extracting to AVRO format.
    #[builder(into)]
    #[serde(rename = "useAvroLogicalTypes")]
    pub r#use_avro_logical_types: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobExtract {
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
                "compression".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#compression,
                )
                .await,
            );
            map.insert(
                "destination_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_format,
                )
                .await,
            );
            map.insert(
                "destination_uris".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_uris,
                )
                .await,
            );
            map.insert(
                "field_delimiter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_delimiter,
                )
                .await,
            );
            map.insert(
                "print_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#print_header,
                )
                .await,
            );
            map.insert(
                "source_model".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_model,
                )
                .await,
            );
            map.insert(
                "source_table".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_table,
                )
                .await,
            );
            map.insert(
                "use_avro_logical_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_avro_logical_types,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobExtract {
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
                    r#compression: {
                        let field_value = match fields_map.get("compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_format: {
                        let field_value = match fields_map.get("destination_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_uris: {
                        let field_value = match fields_map.get("destination_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_delimiter: {
                        let field_value = match fields_map.get("field_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#print_header: {
                        let field_value = match fields_map.get("print_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'print_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_model: {
                        let field_value = match fields_map.get("source_model") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_model' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_table: {
                        let field_value = match fields_map.get("source_table") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_table' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_avro_logical_types: {
                        let field_value = match fields_map.get("use_avro_logical_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_avro_logical_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
