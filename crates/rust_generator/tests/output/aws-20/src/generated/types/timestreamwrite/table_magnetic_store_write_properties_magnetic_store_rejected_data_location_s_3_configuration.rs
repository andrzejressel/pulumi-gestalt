#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
    /// Bucket name of the customer S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Option<String>,
    /// Encryption option for the customer s3 location. Options are S3 server side encryption with an S3-managed key or KMS managed key. Valid values are `SSE_KMS` and `SSE_S3`.
    #[builder(into)]
    #[serde(rename = "encryptionOption")]
    pub r#encryption_option: Option<String>,
    /// KMS key arn for the customer s3 location when encrypting with a KMS managed key.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Object key prefix for the customer S3 location.
    #[builder(into)]
    #[serde(rename = "objectKeyPrefix")]
    pub r#object_key_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("bucket_name".to_string(), self.r#bucket_name.to_pulumi_value().await);
            map.insert("encryption_option".to_string(), self.r#encryption_option.to_pulumi_value().await);
            map.insert("kms_key_id".to_string(), self.r#kms_key_id.to_pulumi_value().await);
            map.insert("object_key_prefix".to_string(), self.r#object_key_prefix.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
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
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption_option: {
                        let field_value = match fields_map.get("encryption_option") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_option' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#object_key_prefix: {
                        let field_value = match fields_map.get("object_key_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_key_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
