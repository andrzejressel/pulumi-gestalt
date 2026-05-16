#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketV2LifecycleRule {
    /// Specifies the number of days after initiating a multipart upload when the multipart upload must be completed.
    #[builder(into)]
    #[serde(rename = "abortIncompleteMultipartUploadDays")]
    pub r#abort_incomplete_multipart_upload_days: Option<i32>,
    /// Specifies lifecycle rule status.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// Specifies a period in the object's expire. See Expiration below for details.
    #[builder(into)]
    #[serde(rename = "expirations")]
    pub r#expirations: Option<Vec<super::super::types::s3::BucketV2LifecycleRuleExpiration>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies when noncurrent object versions expire. See Noncurrent Version Expiration below for details.
    #[builder(into)]
    #[serde(rename = "noncurrentVersionExpirations")]
    pub r#noncurrent_version_expirations: Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionExpiration>>,
    /// Specifies when noncurrent object versions transitions. See Noncurrent Version Transition below for details.
    #[builder(into)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionTransition>>,
    /// Object key prefix identifying one or more objects to which the rule applies.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
    /// Specifies object tags key and value.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<std::collections::HashMap<String, String>>,
    /// Specifies a period in the object's transitions. See Transition below for details.
    #[builder(into)]
    #[serde(rename = "transitions")]
    pub r#transitions: Option<Vec<super::super::types::s3::BucketV2LifecycleRuleTransition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketV2LifecycleRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("abort_incomplete_multipart_upload_days".to_string(), self.r#abort_incomplete_multipart_upload_days.to_pulumi_value().await);
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("expirations".to_string(), self.r#expirations.to_pulumi_value().await);
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("noncurrent_version_expirations".to_string(), self.r#noncurrent_version_expirations.to_pulumi_value().await);
            map.insert("noncurrent_version_transitions".to_string(), self.r#noncurrent_version_transitions.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("tags".to_string(), self.r#tags.to_pulumi_value().await);
            map.insert("transitions".to_string(), self.r#transitions.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketV2LifecycleRule {
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
                    r#abort_incomplete_multipart_upload_days: {
                        let field_value = match fields_map.get("abort_incomplete_multipart_upload_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'abort_incomplete_multipart_upload_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <bool as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#expirations: {
                        let field_value = match fields_map.get("expirations") {
                            Some(value) => value,
                            None => bail!("Missing field 'expirations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketV2LifecycleRuleExpiration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_expirations: {
                        let field_value = match fields_map.get("noncurrent_version_expirations") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrent_version_expirations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionExpiration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_version_transitions: {
                        let field_value = match fields_map.get("noncurrent_version_transitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrent_version_transitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionTransition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<std::collections::HashMap<String, String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#transitions: {
                        let field_value = match fields_map.get("transitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'transitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::s3::BucketV2LifecycleRuleTransition>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
