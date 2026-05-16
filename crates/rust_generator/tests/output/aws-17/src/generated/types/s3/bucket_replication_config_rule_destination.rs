#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketReplicationConfigRuleDestination {
    /// Configuration block that specifies the overrides to use for object owners on replication. See below. Specify this only in a cross-account scenario (where source and destination bucket owners are not the same), and you want to change replica ownership to the AWS account that owns the destination bucket. If this is not specified in the replication configuration, the replicas are owned by same AWS account that owns the source object. Must be used in conjunction with `account` owner override configuration.
    #[builder(into)]
    #[serde(rename = "accessControlTranslation")]
    pub r#access_control_translation: Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationAccessControlTranslation>>,
    /// Account ID to specify the replica ownership. Must be used in conjunction with `access_control_translation` override configuration.
    #[builder(into)]
    #[serde(rename = "account")]
    pub r#account: Option<String>,
    /// ARN of the bucket where you want Amazon S3 to store the results.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// Configuration block that provides information about encryption. See below. If `source_selection_criteria` is specified, you must specify this element.
    #[builder(into)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationEncryptionConfiguration>>,
    /// Configuration block that specifies replication metrics-related settings enabling replication metrics and events. See below.
    #[builder(into)]
    #[serde(rename = "metrics")]
    pub r#metrics: Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationMetrics>>,
    /// Configuration block that specifies S3 Replication Time Control (S3 RTC), including whether S3 RTC is enabled and the time when all objects and operations on objects must be replicated. See below. Replication Time Control must be used in conjunction with `metrics`.
    #[builder(into)]
    #[serde(rename = "replicationTime")]
    pub r#replication_time: Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationReplicationTime>>,
    /// The [storage class](https://docs.aws.amazon.com/AmazonS3/latest/API/API_Destination.html#AmazonS3-Type-Destination-StorageClass) used to store the object. By default, Amazon S3 uses the storage class of the source object to create the object replica.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketReplicationConfigRuleDestination {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("access_control_translation".to_string(), self.r#access_control_translation.to_pulumi_value().await);
            map.insert("account".to_string(), self.r#account.to_pulumi_value().await);
            map.insert("bucket".to_string(), self.r#bucket.to_pulumi_value().await);
            map.insert("encryption_configuration".to_string(), self.r#encryption_configuration.to_pulumi_value().await);
            map.insert("metrics".to_string(), self.r#metrics.to_pulumi_value().await);
            map.insert("replication_time".to_string(), self.r#replication_time.to_pulumi_value().await);
            map.insert("storage_class".to_string(), self.r#storage_class.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketReplicationConfigRuleDestination {
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
                    r#access_control_translation: {
                        let field_value = match fields_map.get("access_control_translation") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_control_translation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationAccessControlTranslation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#account: {
                        let field_value = match fields_map.get("account") {
                            Some(value) => value,
                            None => bail!("Missing field 'account' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bucket: {
                        let field_value = match fields_map.get("bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#encryption_configuration: {
                        let field_value = match fields_map.get("encryption_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationEncryptionConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#metrics: {
                        let field_value = match fields_map.get("metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationMetrics>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#replication_time: {
                        let field_value = match fields_map.get("replication_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'replication_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketReplicationConfigRuleDestinationReplicationTime>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#storage_class: {
                        let field_value = match fields_map.get("storage_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
