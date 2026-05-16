#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationV2Rule {
    /// Configuration block that specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. See below.
    #[builder(into)]
    #[serde(rename = "abortIncompleteMultipartUpload")]
    pub r#abort_incomplete_multipart_upload: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleAbortIncompleteMultipartUpload>>,
    /// Configuration block that specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker. See below.
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleExpiration>>,
    /// Configuration block used to identify objects that a Lifecycle Rule applies to. See below. If not specified, the `rule` will default to using `prefix`.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilter>>,
    /// Unique identifier for the rule. The value cannot be longer than 255 characters.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Configuration block that specifies when noncurrent object versions expire. See below.
    #[builder(into)]
    #[serde(rename = "noncurrentVersionExpiration")]
    pub r#noncurrent_version_expiration: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration>>,
    /// Set of configuration blocks that specify the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. See below.
    #[builder(into)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition>>,
    /// **DEPRECATED** Use `filter` instead. This has been deprecated by Amazon S3. Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if `filter` is not specified.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Whether the rule is currently being applied. Valid values: `Enabled` or `Disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// Set of configuration blocks that specify when an Amazon S3 object transitions to a specified storage class. See below.
    #[builder(into)]
    #[serde(rename = "transitions")]
    pub r#transitions: Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleTransition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLifecycleConfigurationV2Rule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("abort_incomplete_multipart_upload".to_string(), self.r#abort_incomplete_multipart_upload.to_pulumi_value().await);
            map.insert("expiration".to_string(), self.r#expiration.to_pulumi_value().await);
            map.insert("filter".to_string(), self.r#filter.to_pulumi_value().await);
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("noncurrent_version_expiration".to_string(), self.r#noncurrent_version_expiration.to_pulumi_value().await);
            map.insert("noncurrent_version_transitions".to_string(), self.r#noncurrent_version_transitions.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("status".to_string(), self.r#status.to_pulumi_value().await);
            map.insert("transitions".to_string(), self.r#transitions.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLifecycleConfigurationV2Rule {
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
                    r#abort_incomplete_multipart_upload: {
                        let field_value = match fields_map.get("abort_incomplete_multipart_upload") {
                            Some(value) => value,
                            None => bail!("Missing field 'abort_incomplete_multipart_upload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleAbortIncompleteMultipartUpload>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#expiration: {
                        let field_value = match fields_map.get("expiration") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleExpiration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilter>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_expiration: {
                        let field_value = match fields_map.get("noncurrent_version_expiration") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrent_version_expiration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_transitions: {
                        let field_value = match fields_map.get("noncurrent_version_transitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrent_version_transitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#transitions: {
                        let field_value = match fields_map.get("transitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleTransition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
