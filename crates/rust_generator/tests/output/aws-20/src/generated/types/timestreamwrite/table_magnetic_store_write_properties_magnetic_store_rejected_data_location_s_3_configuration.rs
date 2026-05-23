#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
    /// Bucket name of the customer S3 bucket.
    #[builder(into)]
    pub r#bucket_name: Option<String>,
    /// Encryption option for the customer s3 location. Options are S3 server side encryption with an S3-managed key or KMS managed key. Valid values are `SSE_KMS` and `SSE_S3`.
    #[builder(into)]
    pub r#encryption_option: Option<String>,
    /// KMS key arn for the customer s3 location when encrypting with a KMS managed key.
    #[builder(into)]
    pub r#kms_key_id: Option<String>,
    /// Object key prefix for the customer S3 location.
    #[builder(into)]
    pub r#object_key_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
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
                    "bucketName",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "encryptionOption",
                    &self.r#encryption_option,
                ),
                to_pulumi_object_field(
                    "kmsKeyId",
                    &self.r#kms_key_id,
                ),
                to_pulumi_object_field(
                    "objectKeyPrefix",
                    &self.r#object_key_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
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
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucketName") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucketName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_option: {
                        let field_value = match fields_map.get("encryptionOption") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryptionOption' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_id: {
                        let field_value = match fields_map.get("kmsKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'kmsKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_key_prefix: {
                        let field_value = match fields_map.get("objectKeyPrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'objectKeyPrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
