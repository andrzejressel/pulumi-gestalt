#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingEncryptionAtRest {
    /// The encryption-at-rest mode for encrypting Data Catalog data.
    #[builder(into)]
    #[serde(rename = "catalogEncryptionMode")]
    pub r#catalog_encryption_mode: String,
    /// The ARN of the AWS IAM role used for accessing encrypted Data Catalog data.
    #[builder(into)]
    #[serde(rename = "catalogEncryptionServiceRole")]
    pub r#catalog_encryption_service_role: String,
    /// ARN of the AWS KMS key to use for encryption at rest.
    #[builder(into)]
    #[serde(rename = "sseAwsKmsKeyId")]
    pub r#sse_aws_kms_key_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingEncryptionAtRest {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("catalog_encryption_mode".to_string(), self.r#catalog_encryption_mode.to_pulumi_value().await);
            map.insert("catalog_encryption_service_role".to_string(), self.r#catalog_encryption_service_role.to_pulumi_value().await);
            map.insert("sse_aws_kms_key_id".to_string(), self.r#sse_aws_kms_key_id.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDataCatalogEncryptionSettingsDataCatalogEncryptionSettingEncryptionAtRest {
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
                    r#catalog_encryption_mode: {
                        let field_value = match fields_map.get("catalog_encryption_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'catalog_encryption_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#catalog_encryption_service_role: {
                        let field_value = match fields_map.get("catalog_encryption_service_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'catalog_encryption_service_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sse_aws_kms_key_id: {
                        let field_value = match fields_map.get("sse_aws_kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'sse_aws_kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
