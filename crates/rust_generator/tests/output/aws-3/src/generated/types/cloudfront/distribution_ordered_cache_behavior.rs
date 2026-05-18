#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOrderedCacheBehavior {
    /// Controls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin.
    #[builder(into)]
    #[serde(rename = "allowedMethods")]
    pub r#allowed_methods: Vec<String>,
    /// Unique identifier of the cache policy that is attached to the cache behavior. If configuring the `default_cache_behavior` either `cache_policy_id` or `forwarded_values` must be set.
    #[builder(into)]
    #[serde(rename = "cachePolicyId")]
    pub r#cache_policy_id: Option<String>,
    /// Controls whether CloudFront caches the response to requests using the specified HTTP methods.
    #[builder(into)]
    #[serde(rename = "cachedMethods")]
    pub r#cached_methods: Vec<String>,
    /// Whether you want CloudFront to automatically compress content for web requests that include `Accept-Encoding: gzip` in the request header (default: `false`).
    #[builder(into)]
    #[serde(rename = "compress")]
    pub r#compress: Option<bool>,
    /// Default amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request in the absence of an `Cache-Control max-age` or `Expires` header.
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Option<i32>,
    /// Field level encryption configuration ID.
    #[builder(into)]
    #[serde(rename = "fieldLevelEncryptionId")]
    pub r#field_level_encryption_id: Option<String>,
    /// The forwarded values configuration that specifies how CloudFront handles query strings, cookies and headers (maximum one).
    #[builder(into)]
    #[serde(rename = "forwardedValues")]
    pub r#forwarded_values: Option<Box<super::super::types::cloudfront::DistributionOrderedCacheBehaviorForwardedValues>>,
    /// A config block that triggers a cloudfront function with specific actions (maximum 2).
    #[builder(into)]
    #[serde(rename = "functionAssociations")]
    pub r#function_associations: Option<Vec<super::super::types::cloudfront::DistributionOrderedCacheBehaviorFunctionAssociation>>,
    /// A config block that triggers a lambda function with specific actions (maximum 4).
    #[builder(into)]
    #[serde(rename = "lambdaFunctionAssociations")]
    pub r#lambda_function_associations: Option<Vec<super::super::types::cloudfront::DistributionOrderedCacheBehaviorLambdaFunctionAssociation>>,
    /// Maximum amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request to your origin to determine whether the object has been updated. Only effective in the presence of `Cache-Control max-age`, `Cache-Control s-maxage`, and `Expires` headers.
    #[builder(into)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Option<i32>,
    /// Minimum amount of time that you want objects to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated. Defaults to 0 seconds.
    #[builder(into)]
    #[serde(rename = "minTtl")]
    pub r#min_ttl: Option<i32>,
    /// Unique identifier of the origin request policy that is attached to the behavior.
    #[builder(into)]
    #[serde(rename = "originRequestPolicyId")]
    pub r#origin_request_policy_id: Option<String>,
    /// Pattern (for example, `images/*.jpg`) that specifies which requests you want this cache behavior to apply to.
    #[builder(into)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: String,
    /// ARN of the real-time log configuration that is attached to this cache behavior.
    #[builder(into)]
    #[serde(rename = "realtimeLogConfigArn")]
    pub r#realtime_log_config_arn: Option<String>,
    /// Identifier for a response headers policy.
    #[builder(into)]
    #[serde(rename = "responseHeadersPolicyId")]
    pub r#response_headers_policy_id: Option<String>,
    /// Indicates whether you want to distribute media files in Microsoft Smooth Streaming format using the origin that is associated with this cache behavior.
    #[builder(into)]
    #[serde(rename = "smoothStreaming")]
    pub r#smooth_streaming: Option<bool>,
    /// Value of ID for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.
    #[builder(into)]
    #[serde(rename = "targetOriginId")]
    pub r#target_origin_id: String,
    /// List of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
    #[builder(into)]
    #[serde(rename = "trustedKeyGroups")]
    pub r#trusted_key_groups: Option<Vec<String>>,
    /// List of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
    #[builder(into)]
    #[serde(rename = "trustedSigners")]
    pub r#trusted_signers: Option<Vec<String>>,
    /// Use this element to specify the protocol that users can use to access the files in the origin specified by TargetOriginId when a request matches the path pattern in PathPattern. One of `allow-all`, `https-only`, or `redirect-to-https`.
    #[builder(into)]
    #[serde(rename = "viewerProtocolPolicy")]
    pub r#viewer_protocol_policy: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOrderedCacheBehavior {
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
                    "allowed_methods",
                    &self.r#allowed_methods,
                ),
                to_pulumi_object_field(
                    "cache_policy_id",
                    &self.r#cache_policy_id,
                ),
                to_pulumi_object_field(
                    "cached_methods",
                    &self.r#cached_methods,
                ),
                to_pulumi_object_field(
                    "compress",
                    &self.r#compress,
                ),
                to_pulumi_object_field(
                    "default_ttl",
                    &self.r#default_ttl,
                ),
                to_pulumi_object_field(
                    "field_level_encryption_id",
                    &self.r#field_level_encryption_id,
                ),
                to_pulumi_object_field(
                    "forwarded_values",
                    &self.r#forwarded_values,
                ),
                to_pulumi_object_field(
                    "function_associations",
                    &self.r#function_associations,
                ),
                to_pulumi_object_field(
                    "lambda_function_associations",
                    &self.r#lambda_function_associations,
                ),
                to_pulumi_object_field(
                    "max_ttl",
                    &self.r#max_ttl,
                ),
                to_pulumi_object_field(
                    "min_ttl",
                    &self.r#min_ttl,
                ),
                to_pulumi_object_field(
                    "origin_request_policy_id",
                    &self.r#origin_request_policy_id,
                ),
                to_pulumi_object_field(
                    "path_pattern",
                    &self.r#path_pattern,
                ),
                to_pulumi_object_field(
                    "realtime_log_config_arn",
                    &self.r#realtime_log_config_arn,
                ),
                to_pulumi_object_field(
                    "response_headers_policy_id",
                    &self.r#response_headers_policy_id,
                ),
                to_pulumi_object_field(
                    "smooth_streaming",
                    &self.r#smooth_streaming,
                ),
                to_pulumi_object_field(
                    "target_origin_id",
                    &self.r#target_origin_id,
                ),
                to_pulumi_object_field(
                    "trusted_key_groups",
                    &self.r#trusted_key_groups,
                ),
                to_pulumi_object_field(
                    "trusted_signers",
                    &self.r#trusted_signers,
                ),
                to_pulumi_object_field(
                    "viewer_protocol_policy",
                    &self.r#viewer_protocol_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOrderedCacheBehavior {
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
                    r#allowed_methods: {
                        let field_value = match fields_map.get("allowed_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cache_policy_id: {
                        let field_value = match fields_map.get("cache_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cached_methods: {
                        let field_value = match fields_map.get("cached_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'cached_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compress: {
                        let field_value = match fields_map.get("compress") {
                            Some(value) => value,
                            None => bail!("Missing field 'compress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_ttl: {
                        let field_value = match fields_map.get("default_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#field_level_encryption_id: {
                        let field_value = match fields_map.get("field_level_encryption_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_level_encryption_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwarded_values: {
                        let field_value = match fields_map.get("forwarded_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#function_associations: {
                        let field_value = match fields_map.get("function_associations") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_associations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_function_associations: {
                        let field_value = match fields_map.get("lambda_function_associations") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_function_associations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_ttl: {
                        let field_value = match fields_map.get("max_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_ttl: {
                        let field_value = match fields_map.get("min_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_request_policy_id: {
                        let field_value = match fields_map.get("origin_request_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_request_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path_pattern: {
                        let field_value = match fields_map.get("path_pattern") {
                            Some(value) => value,
                            None => bail!("Missing field 'path_pattern' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#realtime_log_config_arn: {
                        let field_value = match fields_map.get("realtime_log_config_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'realtime_log_config_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_headers_policy_id: {
                        let field_value = match fields_map.get("response_headers_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#smooth_streaming: {
                        let field_value = match fields_map.get("smooth_streaming") {
                            Some(value) => value,
                            None => bail!("Missing field 'smooth_streaming' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_origin_id: {
                        let field_value = match fields_map.get("target_origin_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_origin_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_key_groups: {
                        let field_value = match fields_map.get("trusted_key_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_key_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_signers: {
                        let field_value = match fields_map.get("trusted_signers") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_signers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#viewer_protocol_policy: {
                        let field_value = match fields_map.get("viewer_protocol_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'viewer_protocol_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
