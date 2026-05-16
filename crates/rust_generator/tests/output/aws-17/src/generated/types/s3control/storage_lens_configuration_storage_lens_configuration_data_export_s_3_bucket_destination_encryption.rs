#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption {
    /// SSE-KMS encryption. See SSE KMS below for more details.
    #[builder(into)]
    #[serde(rename = "sseKms")]
    pub r#sse_kms: Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseKms>>,
    /// SSE-S3 encryption. An empty configuration block `{}` should be used.
    #[builder(into)]
    #[serde(rename = "sseS3s")]
    pub r#sse_s_3_s: Option<Vec<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseS3>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("sse_kms".to_string(), self.r#sse_kms.to_pulumi_value().await);
            map.insert("sse_s_3_s".to_string(), self.r#sse_s_3_s.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryption {
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
                    r#sse_kms: {
                        let field_value = match fields_map.get("sse_kms") {
                            Some(value) => value,
                            None => bail!("Missing field 'sse_kms' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseKms>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sse_s_3_s: {
                        let field_value = match fields_map.get("sse_s_3_s") {
                            Some(value) => value,
                            None => bail!("Missing field 'sse_s_3_s' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3control::StorageLensConfigurationStorageLensConfigurationDataExportS3BucketDestinationEncryptionSseS3>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
