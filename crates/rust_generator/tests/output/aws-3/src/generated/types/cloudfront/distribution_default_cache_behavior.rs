#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionDefaultCacheBehavior {
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
    pub r#forwarded_values: Option<Box<super::super::types::cloudfront::DistributionDefaultCacheBehaviorForwardedValues>>,
    /// A config block that triggers a cloudfront function with specific actions (maximum 2).
    #[builder(into)]
    #[serde(rename = "functionAssociations")]
    pub r#function_associations: Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorFunctionAssociation>>,
    /// A config block that triggers a lambda function with specific actions (maximum 4).
    #[builder(into)]
    #[serde(rename = "lambdaFunctionAssociations")]
    pub r#lambda_function_associations: Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorLambdaFunctionAssociation>>,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionDefaultCacheBehavior {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("allowed_methods".to_string(), self.r#allowed_methods.to_pulumi_value().await);
            map.insert("cache_policy_id".to_string(), self.r#cache_policy_id.to_pulumi_value().await);
            map.insert("cached_methods".to_string(), self.r#cached_methods.to_pulumi_value().await);
            map.insert("compress".to_string(), self.r#compress.to_pulumi_value().await);
            map.insert("default_ttl".to_string(), self.r#default_ttl.to_pulumi_value().await);
            map.insert("field_level_encryption_id".to_string(), self.r#field_level_encryption_id.to_pulumi_value().await);
            map.insert("forwarded_values".to_string(), self.r#forwarded_values.to_pulumi_value().await);
            map.insert("function_associations".to_string(), self.r#function_associations.to_pulumi_value().await);
            map.insert("lambda_function_associations".to_string(), self.r#lambda_function_associations.to_pulumi_value().await);
            map.insert("max_ttl".to_string(), self.r#max_ttl.to_pulumi_value().await);
            map.insert("min_ttl".to_string(), self.r#min_ttl.to_pulumi_value().await);
            map.insert("origin_request_policy_id".to_string(), self.r#origin_request_policy_id.to_pulumi_value().await);
            map.insert("realtime_log_config_arn".to_string(), self.r#realtime_log_config_arn.to_pulumi_value().await);
            map.insert("response_headers_policy_id".to_string(), self.r#response_headers_policy_id.to_pulumi_value().await);
            map.insert("smooth_streaming".to_string(), self.r#smooth_streaming.to_pulumi_value().await);
            map.insert("target_origin_id".to_string(), self.r#target_origin_id.to_pulumi_value().await);
            map.insert("trusted_key_groups".to_string(), self.r#trusted_key_groups.to_pulumi_value().await);
            map.insert("trusted_signers".to_string(), self.r#trusted_signers.to_pulumi_value().await);
            map.insert("viewer_protocol_policy".to_string(), self.r#viewer_protocol_policy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionDefaultCacheBehavior {
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
                    r#allowed_methods: {
                        let field_value = match fields_map.get("allowed_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cache_policy_id: {
                        let field_value = match fields_map.get("cache_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'cache_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cached_methods: {
                        let field_value = match fields_map.get("cached_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'cached_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#compress: {
                        let field_value = match fields_map.get("compress") {
                            Some(value) => value,
                            None => bail!("Missing field 'compress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#default_ttl: {
                        let field_value = match fields_map.get("default_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#field_level_encryption_id: {
                        let field_value = match fields_map.get("field_level_encryption_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'field_level_encryption_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#forwarded_values: {
                        let field_value = match fields_map.get("forwarded_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarded_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::cloudfront::DistributionDefaultCacheBehaviorForwardedValues>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#function_associations: {
                        let field_value = match fields_map.get("function_associations") {
                            Some(value) => value,
                            None => bail!("Missing field 'function_associations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorFunctionAssociation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda_function_associations: {
                        let field_value = match fields_map.get("lambda_function_associations") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_function_associations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::cloudfront::DistributionDefaultCacheBehaviorLambdaFunctionAssociation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_ttl: {
                        let field_value = match fields_map.get("max_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#min_ttl: {
                        let field_value = match fields_map.get("min_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#origin_request_policy_id: {
                        let field_value = match fields_map.get("origin_request_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_request_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#realtime_log_config_arn: {
                        let field_value = match fields_map.get("realtime_log_config_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'realtime_log_config_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#response_headers_policy_id: {
                        let field_value = match fields_map.get("response_headers_policy_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_headers_policy_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#smooth_streaming: {
                        let field_value = match fields_map.get("smooth_streaming") {
                            Some(value) => value,
                            None => bail!("Missing field 'smooth_streaming' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#target_origin_id: {
                        let field_value = match fields_map.get("target_origin_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_origin_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trusted_key_groups: {
                        let field_value = match fields_map.get("trusted_key_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_key_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#trusted_signers: {
                        let field_value = match fields_map.get("trusted_signers") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_signers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#viewer_protocol_policy: {
                        let field_value = match fields_map.get("viewer_protocol_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'viewer_protocol_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
