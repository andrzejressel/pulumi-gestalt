#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InventoryDestinationBucketEncryption {
    /// Specifies to use server-side encryption with AWS KMS-managed keys to encrypt the inventory file (documented below).
    #[builder(into)]
    #[serde(rename = "sseKms")]
    pub r#sse_kms: Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseKms>>,
    /// Specifies to use server-side encryption with Amazon S3-managed keys (SSE-S3) to encrypt the inventory file.
    #[builder(into)]
    #[serde(rename = "sseS3")]
    pub r#sse_s_3: Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseS3>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InventoryDestinationBucketEncryption {
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
            map.insert("sse_s_3".to_string(), self.r#sse_s_3.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InventoryDestinationBucketEncryption {
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
                        <Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseKms>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#sse_s_3: {
                        let field_value = match fields_map.get("sse_s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 'sse_s_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseS3>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
