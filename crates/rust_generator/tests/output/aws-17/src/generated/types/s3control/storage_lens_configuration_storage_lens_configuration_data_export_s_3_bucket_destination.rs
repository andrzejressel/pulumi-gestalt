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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("account_id".to_string(), self.r#account_id.to_pulumi_value().await);
            map.insert("arn".to_string(), self.r#arn.to_pulumi_value().await);
            map.insert("encryption".to_string(), self.r#encryption.to_pulumi_value().await);
            map.insert("format".to_string(), self.r#format.to_pulumi_value().await);
            map.insert("output_schema_version".to_string(), self.r#output_schema_version.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestination {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#arn: {
                        let field_value = match fields_map.get("arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption: {
                        let field_value = match fields_map.get("encryption") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_schema_version: {
                        let field_value = match fields_map.get("output_schema_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_schema_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
