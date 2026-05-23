#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketV2ReplicationConfigurationRuleDestination {
    /// Specifies the overrides to use for object owners on replication (documented below). Must be used in conjunction with `account_id` owner override configuration.
    #[builder(into)]
    pub r#access_control_translations: Option<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRuleDestinationAccessControlTranslation>>,
    /// Account ID to use for overriding the object owner on replication. Must be used in conjunction with `access_control_translation` override configuration.
    #[builder(into)]
    pub r#account_id: Option<String>,
    /// ARN of the S3 bucket where you want Amazon S3 to store replicas of the object identified by the rule.
    #[builder(into)]
    pub r#bucket: String,
    /// Enables replication metrics (required for S3 RTC) (documented below).
    #[builder(into)]
    pub r#metrics: Option<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRuleDestinationMetric>>,
    /// Destination KMS encryption key ARN for SSE-KMS replication. Must be used in conjunction with
    /// `sse_kms_encrypted_objects` source selection criteria.
    #[builder(into)]
    pub r#replica_kms_key_id: Option<String>,
    /// Enables S3 Replication Time Control (S3 RTC) (documented below).
    #[builder(into)]
    pub r#replication_times: Option<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRuleDestinationReplicationTime>>,
    /// The [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Destination.html#AmazonS3-Type-Destination-StorageClass) used to store the object. By default, Amazon S3 uses the storage class of the source object to create the object replica.
    #[builder(into)]
    pub r#storage_class: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketV2ReplicationConfigurationRuleDestination {
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
                    "accessControlTranslations",
                    &self.r#access_control_translations,
                ),
                to_pulumi_object_field(
                    "accountId",
                    &self.r#account_id,
                ),
                to_pulumi_object_field(
                    "bucket",
                    &self.r#bucket,
                ),
                to_pulumi_object_field(
                    "metrics",
                    &self.r#metrics,
                ),
                to_pulumi_object_field(
                    "replicaKmsKeyId",
                    &self.r#replica_kms_key_id,
                ),
                to_pulumi_object_field(
                    "replicationTimes",
                    &self.r#replication_times,
                ),
                to_pulumi_object_field(
                    "storageClass",
                    &self.r#storage_class,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketV2ReplicationConfigurationRuleDestination {
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
                    r#access_control_translations: {
                        let field_value = match fields_map.get("accessControlTranslations") {
                            Some(value) => value,
                            None => bail!("Missing field 'accessControlTranslations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account_id: {
                        let field_value = match fields_map.get("accountId") {
                            Some(value) => value,
                            None => bail!("Missing field 'accountId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket: {
                        let field_value = match fields_map.get("bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metrics: {
                        let field_value = match fields_map.get("metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replica_kms_key_id: {
                        let field_value = match fields_map.get("replicaKmsKeyId") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicaKmsKeyId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replication_times: {
                        let field_value = match fields_map.get("replicationTimes") {
                            Some(value) => value,
                            None => bail!("Missing field 'replicationTimes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_class: {
                        let field_value = match fields_map.get("storageClass") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageClass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
