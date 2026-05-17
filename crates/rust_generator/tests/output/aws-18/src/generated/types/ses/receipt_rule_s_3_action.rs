#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReceiptRuleS3Action {
    /// The name of the S3 bucket
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// The ARN of the IAM role to be used by Amazon Simple Email Service while writing to the Amazon S3 bucket, optionally encrypting your mail via the provided customer managed key, and publishing to the Amazon SNS topic
    #[builder(into)]
    #[serde(rename = "iamRoleArn")]
    pub r#iam_role_arn: Option<String>,
    /// The ARN of the KMS key
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// The key prefix of the S3 bucket
    #[builder(into)]
    #[serde(rename = "objectKeyPrefix")]
    pub r#object_key_prefix: Option<String>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: i32,
    /// The ARN of an SNS topic to notify
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReceiptRuleS3Action {
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
                "bucket_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_name,
                )
                .await,
            );
            map.insert(
                "iam_role_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#iam_role_arn,
                )
                .await,
            );
            map.insert(
                "kms_key_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kms_key_arn,
                )
                .await,
            );
            map.insert(
                "object_key_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object_key_prefix,
                )
                .await,
            );
            map.insert(
                "position".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#position,
                )
                .await,
            );
            map.insert(
                "topic_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReceiptRuleS3Action {
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
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#iam_role_arn: {
                        let field_value = match fields_map.get("iam_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'iam_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kms_key_arn: {
                        let field_value = match fields_map.get("kms_key_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'kms_key_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_key_prefix: {
                        let field_value = match fields_map.get("object_key_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_key_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#position: {
                        let field_value = match fields_map.get("position") {
                            Some(value) => value,
                            None => bail!("Missing field 'position' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_arn: {
                        let field_value = match fields_map.get("topic_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
