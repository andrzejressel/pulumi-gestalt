#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination {
    /// The account ID of the owner of the S3 Storage Lens metrics export bucket.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// The Amazon Resource Name (ARN) of the bucket.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Encryption of the metrics exports in this bucket. See Encryption below for more details.
    #[builder(into)]
    #[serde(rename = "encryption")]
    pub r#encryption: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption>>,
    /// The export format. Valid values: `CSV`, `Parquet`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    /// The schema version of the export file. Valid values: `V_1`.
    #[builder(into)]
    #[serde(rename = "outputSchemaVersion")]
    pub r#output_schema_version: String,
    /// The prefix of the destination bucket where the metrics export will be delivered.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_id,
                )
                .await,
            );
            map.insert(
                "arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#arn,
                )
                .await,
            );
            map.insert(
                "encryption".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption,
                )
                .await,
            );
            map.insert(
                "format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#format,
                )
                .await,
            );
            map.insert(
                "output_schema_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_schema_version,
                )
                .await,
            );
            map.insert(
                "prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#prefix,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination {
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
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption: {
                        let field_value = match fields_map.get("encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_schema_version: {
                        let field_value = match fields_map.get("output_schema_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_schema_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
