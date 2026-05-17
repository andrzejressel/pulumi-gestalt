#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketV2ServerSideEncryptionConfigurationRule {
    /// Single object for setting server-side encryption by default. (documented below)
    #[builder(into)]
    #[serde(rename = "applyServerSideEncryptionByDefaults")]
    pub r#apply_server_side_encryption_by_defaults: Vec<super::super::types::s3::BucketV2ServerSideEncryptionConfigurationRuleApplyServerSideEncryptionByDefault>,
    /// Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
    #[builder(into)]
    #[serde(rename = "bucketKeyEnabled")]
    pub r#bucket_key_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketV2ServerSideEncryptionConfigurationRule {
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
                "apply_server_side_encryption_by_defaults".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#apply_server_side_encryption_by_defaults,
                )
                .await,
            );
            map.insert(
                "bucket_key_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_key_enabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketV2ServerSideEncryptionConfigurationRule {
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
                    r#apply_server_side_encryption_by_defaults: {
                        let field_value = match fields_map.get("apply_server_side_encryption_by_defaults") {
                            Some(value) => value,
                            None => bail!("Missing field 'apply_server_side_encryption_by_defaults' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_key_enabled: {
                        let field_value = match fields_map.get("bucket_key_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_key_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
