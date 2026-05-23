#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationV2Rule {
    /// Configuration block that specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. See below.
    #[builder(into)]
    pub r#abort_incomplete_multipart_upload: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleAbortIncompleteMultipartUpload>>,
    /// Configuration block that specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker. See below.
    #[builder(into)]
    pub r#expiration: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleExpiration>>,
    /// Configuration block used to identify objects that a Lifecycle Rule applies to. See below. If not specified, the `rule` will default to using `prefix`.
    #[builder(into)]
    pub r#filter: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilter>>,
    /// Unique identifier for the rule. The value cannot be longer than 255 characters.
    #[builder(into)]
    pub r#id: String,
    /// Configuration block that specifies when noncurrent object versions expire. See below.
    #[builder(into)]
    pub r#noncurrent_version_expiration: Option<Box<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration>>,
    /// Set of configuration blocks that specify the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. See below.
    #[builder(into)]
    pub r#noncurrent_version_transitions: Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition>>,
    /// **DEPRECATED** Use `filter` instead. This has been deprecated by Amazon S3. Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if `filter` is not specified.
    #[builder(into)]
    pub r#prefix: Option<String>,
    /// Whether the rule is currently being applied. Valid values: `Enabled` or `Disabled`.
    #[builder(into)]
    pub r#status: String,
    /// Set of configuration blocks that specify when an Amazon S3 object transitions to a specified storage class. See below.
    #[builder(into)]
    pub r#transitions: Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleTransition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLifecycleConfigurationV2Rule {
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
                    "abortIncompleteMultipartUpload",
                    &self.r#abort_incomplete_multipart_upload,
                ),
                to_pulumi_object_field(
                    "expiration",
                    &self.r#expiration,
                ),
                to_pulumi_object_field(
                    "filter",
                    &self.r#filter,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "noncurrentVersionExpiration",
                    &self.r#noncurrent_version_expiration,
                ),
                to_pulumi_object_field(
                    "noncurrentVersionTransitions",
                    &self.r#noncurrent_version_transitions,
                ),
                to_pulumi_object_field(
                    "prefix",
                    &self.r#prefix,
                ),
                to_pulumi_object_field(
                    "status",
                    &self.r#status,
                ),
                to_pulumi_object_field(
                    "transitions",
                    &self.r#transitions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLifecycleConfigurationV2Rule {
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
                    r#abort_incomplete_multipart_upload: {
                        let field_value = match fields_map.get("abortIncompleteMultipartUpload") {
                            Some(value) => value,
                            None => bail!("Missing field 'abortIncompleteMultipartUpload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expiration: {
                        let field_value = match fields_map.get("expiration") {
                            Some(value) => value,
                            None => bail!("Missing field 'expiration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter: {
                        let field_value = match fields_map.get("filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_expiration: {
                        let field_value = match fields_map.get("noncurrentVersionExpiration") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrentVersionExpiration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_transitions: {
                        let field_value = match fields_map.get("noncurrentVersionTransitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrentVersionTransitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transitions: {
                        let field_value = match fields_map.get("transitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
