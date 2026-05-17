#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableImportTable {
    /// Type of compression to be used on the input coming from the imported table.
    /// Valid values are `GZIP`, `ZSTD` and `NONE`.
    #[builder(into)]
    #[serde(rename = "inputCompressionType")]
    pub r#input_compression_type: Option<String>,
    /// The format of the source data.
    /// Valid values are `CSV`, `DYNAMODB_JSON`, and `ION`.
    #[builder(into)]
    #[serde(rename = "inputFormat")]
    pub r#input_format: String,
    /// Describe the format options for the data that was imported into the target table.
    /// There is one value, `csv`.
    /// See below.
    #[builder(into)]
    #[serde(rename = "inputFormatOptions")]
    pub r#input_format_options: Option<Box<super::super::types::dynamodb::TableImportTableInputFormatOptions>>,
    /// Values for the S3 bucket the source file is imported from.
    /// See below.
    #[builder(into)]
    #[serde(rename = "s3BucketSource")]
    pub r#s_3_bucket_source: Box<super::super::types::dynamodb::TableImportTableS3BucketSource>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableImportTable {
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
                "input_compression_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_compression_type,
                )
                .await,
            );
            map.insert(
                "input_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_format,
                )
                .await,
            );
            map.insert(
                "input_format_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_format_options,
                )
                .await,
            );
            map.insert(
                "s_3_bucket_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_bucket_source,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableImportTable {
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
                    r#input_compression_type: {
                        let field_value = match fields_map.get("input_compression_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_compression_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_format: {
                        let field_value = match fields_map.get("input_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_format_options: {
                        let field_value = match fields_map.get("input_format_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_format_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket_source: {
                        let field_value = match fields_map.get("s_3_bucket_source") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
