#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[builder(into)]
    #[serde(rename = "batchPrefix")]
    pub r#batch_prefix: Option<String>,
    /// String to be appended after each batch.
    #[builder(into)]
    #[serde(rename = "batchSuffix")]
    pub r#batch_suffix: Option<String>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cve20214428")]
    pub r#cve_20214428: Option<bool>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[builder(into)]
    #[serde(rename = "fieldDelimiter")]
    pub r#field_delimiter: Option<String>,
    /// List of field names to be included in the Logpush output.
    #[builder(into)]
    #[serde(rename = "fieldNames")]
    pub r#field_names: Option<Vec<String>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[builder(into)]
    #[serde(rename = "outputType")]
    pub r#output_type: Option<String>,
    /// String to be inserted in-between the records as separator.
    #[builder(into)]
    #[serde(rename = "recordDelimiter")]
    pub r#record_delimiter: Option<String>,
    /// String to be prepended before each record. Defaults to `{`.
    #[builder(into)]
    #[serde(rename = "recordPrefix")]
    pub r#record_prefix: Option<String>,
    /// String to be appended after each record. Defaults to `}
    /// `.
    #[builder(into)]
    #[serde(rename = "recordSuffix")]
    pub r#record_suffix: Option<String>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[builder(into)]
    #[serde(rename = "recordTemplate")]
    pub r#record_template: Option<String>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Option<f64>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[builder(into)]
    #[serde(rename = "timestampFormat")]
    pub r#timestamp_format: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LogpushJobOutputOptions {
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
                "batch_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_prefix,
                )
                .await,
            );
            map.insert(
                "batch_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#batch_suffix,
                )
                .await,
            );
            map.insert(
                "cve_20214428".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cve_20214428,
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
                "field_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#field_names,
                )
                .await,
            );
            map.insert(
                "output_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_type,
                )
                .await,
            );
            map.insert(
                "record_delimiter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_delimiter,
                )
                .await,
            );
            map.insert(
                "record_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_prefix,
                )
                .await,
            );
            map.insert(
                "record_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_suffix,
                )
                .await,
            );
            map.insert(
                "record_template".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_template,
                )
                .await,
            );
            map.insert(
                "sample_rate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sample_rate,
                )
                .await,
            );
            map.insert(
                "timestamp_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timestamp_format,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LogpushJobOutputOptions {
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
                    r#batch_prefix: {
                        let field_value = match fields_map.get("batch_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch_suffix: {
                        let field_value = match fields_map.get("batch_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'batch_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cve_20214428: {
                        let field_value = match fields_map.get("cve_20214428") {
                            Some(value) => value,
                            None => bail!("Missing field 'cve_20214428' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#field_names: {
                        let field_value = match fields_map.get("field_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_type: {
                        let field_value = match fields_map.get("output_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_delimiter: {
                        let field_value = match fields_map.get("record_delimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_delimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_prefix: {
                        let field_value = match fields_map.get("record_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_suffix: {
                        let field_value = match fields_map.get("record_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_template: {
                        let field_value = match fields_map.get("record_template") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_template' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_rate: {
                        let field_value = match fields_map.get("sample_rate") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_rate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_format: {
                        let field_value = match fields_map.get("timestamp_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestamp_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
