#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LogpushJobOutputOptions {
    /// String to be prepended before each batch.
    #[builder(into)]
    pub r#batch_prefix: Option<String>,
    /// String to be appended after each batch.
    #[builder(into)]
    pub r#batch_suffix: Option<String>,
    /// Mitigation for CVE-2021-44228. If set to true, will cause all occurrences of ${ in the generated files to be replaced with x{. Defaults to `false`.
    #[builder(into)]
    pub r#cve_20214428: Option<bool>,
    /// String to join fields. This field be ignored when record_template is set. Defaults to `,`.
    #[builder(into)]
    pub r#field_delimiter: Option<String>,
    /// List of field names to be included in the Logpush output.
    #[builder(into)]
    pub r#field_names: Option<Vec<String>>,
    /// Specifies the output type. Available values: `ndjson`, `csv`. Defaults to `ndjson`.
    #[builder(into)]
    pub r#output_type: Option<String>,
    /// String to be inserted in-between the records as separator.
    #[builder(into)]
    pub r#record_delimiter: Option<String>,
    /// String to be prepended before each record. Defaults to `{`.
    #[builder(into)]
    pub r#record_prefix: Option<String>,
    /// String to be appended after each record. Defaults to `}
    /// `.
    #[builder(into)]
    pub r#record_suffix: Option<String>,
    /// String to use as template for each record instead of the default comma-separated list.
    #[builder(into)]
    pub r#record_template: Option<String>,
    /// Specifies the sampling rate. Defaults to `1`.
    #[builder(into)]
    pub r#sample_rate: Option<f64>,
    /// Specifies the format for timestamps. Available values: `unixnano`, `unix`, `rfc3339`. Defaults to `unixnano`.
    #[builder(into)]
    pub r#timestamp_format: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LogpushJobOutputOptions {
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
                    "batchPrefix",
                    &self.r#batch_prefix,
                ),
                to_pulumi_object_field(
                    "batchSuffix",
                    &self.r#batch_suffix,
                ),
                to_pulumi_object_field(
                    "cve20214428",
                    &self.r#cve_20214428,
                ),
                to_pulumi_object_field(
                    "fieldDelimiter",
                    &self.r#field_delimiter,
                ),
                to_pulumi_object_field(
                    "fieldNames",
                    &self.r#field_names,
                ),
                to_pulumi_object_field(
                    "outputType",
                    &self.r#output_type,
                ),
                to_pulumi_object_field(
                    "recordDelimiter",
                    &self.r#record_delimiter,
                ),
                to_pulumi_object_field(
                    "recordPrefix",
                    &self.r#record_prefix,
                ),
                to_pulumi_object_field(
                    "recordSuffix",
                    &self.r#record_suffix,
                ),
                to_pulumi_object_field(
                    "recordTemplate",
                    &self.r#record_template,
                ),
                to_pulumi_object_field(
                    "sampleRate",
                    &self.r#sample_rate,
                ),
                to_pulumi_object_field(
                    "timestampFormat",
                    &self.r#timestamp_format,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("batchPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'batchPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#batch_suffix: {
                        let field_value = match fields_map.get("batchSuffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'batchSuffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cve_20214428: {
                        let field_value = match fields_map.get("cve20214428") {
                            Some(value) => value,
                            None => bail!("Missing field 'cve20214428' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#field_names: {
                        let field_value = match fields_map.get("fieldNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'fieldNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_type: {
                        let field_value = match fields_map.get("outputType") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_delimiter: {
                        let field_value = match fields_map.get("recordDelimiter") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordDelimiter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_prefix: {
                        let field_value = match fields_map.get("recordPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_suffix: {
                        let field_value = match fields_map.get("recordSuffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordSuffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_template: {
                        let field_value = match fields_map.get("recordTemplate") {
                            Some(value) => value,
                            None => bail!("Missing field 'recordTemplate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_rate: {
                        let field_value = match fields_map.get("sampleRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'sampleRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timestamp_format: {
                        let field_value = match fields_map.get("timestampFormat") {
                            Some(value) => value,
                            None => bail!("Missing field 'timestampFormat' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
