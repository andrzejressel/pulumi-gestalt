#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOrigin {
    /// Number of times that CloudFront attempts to connect to the origin. Must be between 1-3. Defaults to 3.
    #[builder(into)]
    #[serde(rename = "connectionAttempts")]
    pub r#connection_attempts: Option<i32>,
    /// Number of seconds that CloudFront waits when trying to establish a connection to the origin. Must be between 1-10. Defaults to 10.
    #[builder(into)]
    #[serde(rename = "connectionTimeout")]
    pub r#connection_timeout: Option<i32>,
    /// One or more sub-resources with `name` and `value` parameters that specify header data that will be sent to the origin (multiples allowed).
    #[builder(into)]
    #[serde(rename = "customHeaders")]
    pub r#custom_headers: Option<Vec<super::super::types::cloudfront::DistributionOriginCustomHeader>>,
    /// The CloudFront custom origin configuration information. If an S3 origin is required, use `origin_access_control_id` or `s3_origin_config` instead.
    #[builder(into)]
    #[serde(rename = "customOriginConfig")]
    pub r#custom_origin_config: Option<Box<super::super::types::cloudfront::DistributionOriginCustomOriginConfig>>,
    /// Domain name corresponding to the distribution. For example: `d604721fxaaqy9.cloudfront.net`.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// Unique identifier of a [CloudFront origin access control][8] for this origin.
    #[builder(into)]
    #[serde(rename = "originAccessControlId")]
    pub r#origin_access_control_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "originId")]
    pub r#origin_id: String,
    /// Optional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin.
    #[builder(into)]
    #[serde(rename = "originPath")]
    pub r#origin_path: Option<String>,
    /// CloudFront Origin Shield configuration information. Using Origin Shield can help reduce the load on your origin. For more information, see [Using Origin Shield](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/origin-shield.html) in the Amazon CloudFront Developer Guide.
    #[builder(into)]
    #[serde(rename = "originShield")]
    pub r#origin_shield: Option<Box<super::super::types::cloudfront::DistributionOriginOriginShield>>,
    /// CloudFront S3 origin configuration information. If a custom origin is required, use `custom_origin_config` instead.
    #[builder(into)]
    #[serde(rename = "s3OriginConfig")]
    pub r#s_3_origin_config: Option<Box<super::super::types::cloudfront::DistributionOriginS3OriginConfig>>,
    /// The VPC origin configuration.
    #[builder(into)]
    #[serde(rename = "vpcOriginConfig")]
    pub r#vpc_origin_config: Option<Box<super::super::types::cloudfront::DistributionOriginVpcOriginConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOrigin {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "connection_attempts",
                    &self.r#connection_attempts,
                ),
                to_pulumi_object_field(
                    "connection_timeout",
                    &self.r#connection_timeout,
                ),
                to_pulumi_object_field(
                    "custom_headers",
                    &self.r#custom_headers,
                ),
                to_pulumi_object_field(
                    "custom_origin_config",
                    &self.r#custom_origin_config,
                ),
                to_pulumi_object_field(
                    "domain_name",
                    &self.r#domain_name,
                ),
                to_pulumi_object_field(
                    "origin_access_control_id",
                    &self.r#origin_access_control_id,
                ),
                to_pulumi_object_field(
                    "origin_id",
                    &self.r#origin_id,
                ),
                to_pulumi_object_field(
                    "origin_path",
                    &self.r#origin_path,
                ),
                to_pulumi_object_field(
                    "origin_shield",
                    &self.r#origin_shield,
                ),
                to_pulumi_object_field(
                    "s_3_origin_config",
                    &self.r#s_3_origin_config,
                ),
                to_pulumi_object_field(
                    "vpc_origin_config",
                    &self.r#vpc_origin_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOrigin {
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
                    r#connection_attempts: {
                        let field_value = match fields_map.get("connection_attempts") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_attempts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_timeout: {
                        let field_value = match fields_map.get("connection_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_headers: {
                        let field_value = match fields_map.get("custom_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_origin_config: {
                        let field_value = match fields_map.get("custom_origin_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_origin_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#domain_name: {
                        let field_value = match fields_map.get("domain_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'domain_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_access_control_id: {
                        let field_value = match fields_map.get("origin_access_control_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_access_control_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_id: {
                        let field_value = match fields_map.get("origin_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_path: {
                        let field_value = match fields_map.get("origin_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_shield: {
                        let field_value = match fields_map.get("origin_shield") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_shield' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_origin_config: {
                        let field_value = match fields_map.get("s_3_origin_config") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_origin_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_origin_config: {
                        let field_value = match fields_map.get("vpc_origin_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_origin_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
