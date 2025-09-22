#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
