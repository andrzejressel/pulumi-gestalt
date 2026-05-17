#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketReplicationConfigurationRuleDestination {
    /// Specifies the overrides to use for object owners on replication. Must be used in conjunction with `account_id` owner override configuration.
    #[builder(into)]
    #[serde(rename = "accessControlTranslation")]
    pub r#access_control_translation: Option<Box<super::super::types::s3::BucketReplicationConfigurationRuleDestinationAccessControlTranslation>>,
    /// The Account ID to use for overriding the object owner on replication. Must be used in conjunction with `access_control_translation` override configuration.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// The ARN of the S3 bucket where you want Amazon S3 to store replicas of the object identified by the rule.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Enables replication metrics (required for S3 RTC) (documented below).
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Box<super::super::types::s3::BucketReplicationConfigurationRuleDestinationMetrics>>,
    /// Destination KMS encryption key ARN for SSE-KMS replication. Must be used in conjunction with
    /// `sse_kms_encrypted_objects` source selection criteria.
    #[builder(into)]
    #[serde(rename = "replicaKmsKeyId")]
    pub r#replica_kms_key_id: Option<String>,
    /// Enables S3 Replication Time Control (S3 RTC) (documented below).
    #[builder(into)]
    #[serde(rename = "replicationTime")]
    pub r#replication_time: Option<Box<super::super::types::s3::BucketReplicationConfigurationRuleDestinationReplicationTime>>,
    /// The [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Destination.html#AmazonS3-Type-Destination-StorageClass) used to store the object. By default, Amazon S3 uses the storage class of the source object to create the object replica.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketReplicationConfigurationRuleDestination {
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
                "access_control_translation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_control_translation,
                )
                .await,
            );
            map.insert(
                "account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#account_id,
                )
                .await,
            );
            map.insert(
                "bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket,
                )
                .await,
            );
            map.insert(
                "metrics".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metrics,
                )
                .await,
            );
            map.insert(
                "replica_kms_key_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replica_kms_key_id,
                )
                .await,
            );
            map.insert(
                "replication_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replication_time,
                )
                .await,
            );
            map.insert(
                "storage_class".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_class,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketReplicationConfigurationRuleDestination {
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
                    r#access_control_translation: {
                        let field_value = match fields_map.get("access_control_translation") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_translation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#account_id: {
                        let field_value = match fields_map.get("account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("replica_kms_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'replica_kms_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replication_time: {
                        let field_value = match fields_map.get("replication_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_class: {
                        let field_value = match fields_map.get("storage_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
