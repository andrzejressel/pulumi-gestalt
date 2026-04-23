/// The provider type for the aws package. By default, resources use package-wide configuration
/// settings, however an explicit `Provider` instance may be created and passed during resource
/// construction to achieve fine-grained programmatic control over provider settings. See the
/// [documentation](https://www.pulumi.com/docs/reference/programming-model/#providers) for more information.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
#[derive(pulumi_gestalt_rust::__private::bon::Builder)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProviderArgs {
    /// The access key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
    #[builder(into, default)]
    pub access_key: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub allowed_account_ids: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    #[builder(into, default)]
    pub assume_role: pulumi_gestalt_rust::Input<
        Option<super::types::ProviderAssumeRole>,
    >,
    #[builder(into, default)]
    pub assume_role_with_web_identity: pulumi_gestalt_rust::Input<
        Option<super::types::ProviderAssumeRoleWithWebIdentity>,
    >,
    /// File containing custom root and intermediate certificates. Can also be configured using the `AWS_CA_BUNDLE` environment
    /// variable. (Setting `ca_bundle` in the shared config file is not supported.)
    #[builder(into, default)]
    pub custom_ca_bundle: pulumi_gestalt_rust::Input<Option<String>>,
    /// Configuration block with settings to default resource tags across all resources.
    #[builder(into, default)]
    pub default_tags: pulumi_gestalt_rust::Input<
        Option<super::types::ProviderDefaultTags>,
    >,
    /// Address of the EC2 metadata service endpoint to use. Can also be configured using the
    /// `AWS_EC2_METADATA_SERVICE_ENDPOINT` environment variable.
    #[builder(into, default)]
    pub ec2_metadata_service_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    /// Protocol to use with EC2 metadata service endpoint.Valid values are `IPv4` and `IPv6`. Can also be configured using the
    /// `AWS_EC2_METADATA_SERVICE_ENDPOINT_MODE` environment variable.
    #[builder(into, default)]
    pub ec2_metadata_service_endpoint_mode: pulumi_gestalt_rust::Input<Option<String>>,
    #[builder(into, default)]
    pub endpoints: pulumi_gestalt_rust::Input<
        Option<Vec<super::types::ProviderEndpoint>>,
    >,
    #[builder(into, default)]
    pub forbidden_account_ids: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    /// URL of a proxy to use for HTTP requests when accessing the AWS API. Can also be set using the `HTTP_PROXY` or
    /// `http_proxy` environment variables.
    #[builder(into, default)]
    pub http_proxy: pulumi_gestalt_rust::Input<Option<String>>,
    /// URL of a proxy to use for HTTPS requests when accessing the AWS API. Can also be set using the `HTTPS_PROXY` or
    /// `https_proxy` environment variables.
    #[builder(into, default)]
    pub https_proxy: pulumi_gestalt_rust::Input<Option<String>>,
    /// Configuration block with settings to ignore resource tags across all resources.
    #[builder(into, default)]
    pub ignore_tags: pulumi_gestalt_rust::Input<
        Option<super::types::ProviderIgnoreTags>,
    >,
    /// Explicitly allow the provider to perform "insecure" SSL requests. If omitted, default value is `false`
    #[builder(into, default)]
    pub insecure: pulumi_gestalt_rust::Input<Option<bool>>,
    /// The maximum number of times an AWS API request is being executed. If the API request still fails, an error is thrown.
    #[builder(into, default)]
    pub max_retries: pulumi_gestalt_rust::Input<Option<i32>>,
    /// Comma-separated list of hosts that should not use HTTP or HTTPS proxies. Can also be set using the `NO_PROXY` or
    /// `no_proxy` environment variables.
    #[builder(into, default)]
    pub no_proxy: pulumi_gestalt_rust::Input<Option<String>>,
    /// The profile for API operations. If not set, the default profile created with `aws configure` will be used.
    #[builder(into, default)]
    pub profile: pulumi_gestalt_rust::Input<Option<String>>,
    /// The region where AWS operations will take place. Examples are us-east-1, us-west-2, etc.
    #[builder(into, default)]
    pub region: pulumi_gestalt_rust::Input<Option<String>>,
    /// Specifies how retries are attempted. Valid values are `standard` and `adaptive`. Can also be configured using the
    /// `AWS_RETRY_MODE` environment variable.
    #[builder(into, default)]
    pub retry_mode: pulumi_gestalt_rust::Input<Option<String>>,
    /// Specifies whether S3 API calls in the `us-east-1` region use the legacy global endpoint or a regional endpoint. Valid
    /// values are `legacy` or `regional`. Can also be configured using the `AWS_S3_US_EAST_1_REGIONAL_ENDPOINT` environment
    /// variable or the `s3_us_east_1_regional_endpoint` shared config file parameter
    #[builder(into, default)]
    pub s3_us_east1_regional_endpoint: pulumi_gestalt_rust::Input<Option<String>>,
    /// Set this to true to enable the request to use path-style addressing, i.e., https://s3.amazonaws.com/BUCKET/KEY. By
    /// default, the S3 client will use virtual hosted bucket addressing when possible (https://BUCKET.s3.amazonaws.com/KEY).
    /// Specific to the Amazon S3 service.
    #[builder(into, default)]
    pub s3_use_path_style: pulumi_gestalt_rust::Input<Option<bool>>,
    /// The secret key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
    #[builder(into, default)]
    pub secret_key: pulumi_gestalt_rust::Input<Option<String>>,
    /// List of paths to shared config files. If not set, defaults to [~/.aws/config].
    #[builder(into, default)]
    pub shared_config_files: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    /// List of paths to shared credentials files. If not set, defaults to [~/.aws/credentials].
    #[builder(into, default)]
    pub shared_credentials_files: pulumi_gestalt_rust::Input<Option<Vec<String>>>,
    /// Skip the credentials validation via STS API. Used for AWS API implementations that do not have STS
    /// available/implemented.
    #[builder(into, default)]
    pub skip_credentials_validation: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Skip the AWS Metadata API check. Used for AWS API implementations that do not have a metadata api endpoint.
    #[builder(into, default)]
    pub skip_metadata_api_check: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Skip static validation of region name. Used by users of alternative AWS-like APIs or users w/ access to regions that are
    /// not public (yet).
    #[builder(into, default)]
    pub skip_region_validation: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Skip requesting the account ID. Used for AWS API implementations that do not have IAM/STS API and/or metadata API.
    #[builder(into, default)]
    pub skip_requesting_account_id: pulumi_gestalt_rust::Input<Option<bool>>,
    /// The region where AWS STS operations will take place. Examples are us-east-1 and us-west-2.
    #[builder(into, default)]
    pub sts_region: pulumi_gestalt_rust::Input<Option<String>>,
    /// session token. A session token is only required if you are using temporary security credentials.
    #[builder(into, default)]
    pub token: pulumi_gestalt_rust::Input<Option<String>>,
    /// The capacity of the AWS SDK's token bucket rate limiter.
    #[builder(into, default)]
    pub token_bucket_rate_limiter_capacity: pulumi_gestalt_rust::Input<Option<i32>>,
    /// Resolve an endpoint with DualStack capability
    #[builder(into, default)]
    pub use_dualstack_endpoint: pulumi_gestalt_rust::Input<Option<bool>>,
    /// Resolve an endpoint with FIPS capability
    #[builder(into, default)]
    pub use_fips_endpoint: pulumi_gestalt_rust::Input<Option<bool>>,
}
#[allow(dead_code)]
pub struct ProviderResult {
    /// Pulumi URN is the stable logical identity of this provider resource in the Pulumi stack.
    pub urn: pulumi_gestalt_rust::Output<String>,
    /// Pulumi ID is the unique identifier assigned by the provider to this resource.
    pub id: pulumi_gestalt_rust::Output<String>,
    /// Pulumi Provider ID is the combination of URN and ID. It is used when creating a resource.
    pub provider_id: pulumi_gestalt_rust::Output<String>,
    /// The access key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
    pub access_key: pulumi_gestalt_rust::Output<Option<String>>,
    pub allowed_account_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    pub assume_role: pulumi_gestalt_rust::Output<
        Option<super::types::ProviderAssumeRole>,
    >,
    pub assume_role_with_web_identity: pulumi_gestalt_rust::Output<
        Option<super::types::ProviderAssumeRoleWithWebIdentity>,
    >,
    /// File containing custom root and intermediate certificates. Can also be configured using the `AWS_CA_BUNDLE` environment
    /// variable. (Setting `ca_bundle` in the shared config file is not supported.)
    pub custom_ca_bundle: pulumi_gestalt_rust::Output<Option<String>>,
    /// Configuration block with settings to default resource tags across all resources.
    pub default_tags: pulumi_gestalt_rust::Output<
        Option<super::types::ProviderDefaultTags>,
    >,
    /// Address of the EC2 metadata service endpoint to use. Can also be configured using the
    /// `AWS_EC2_METADATA_SERVICE_ENDPOINT` environment variable.
    pub ec2_metadata_service_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    /// Protocol to use with EC2 metadata service endpoint.Valid values are `IPv4` and `IPv6`. Can also be configured using the
    /// `AWS_EC2_METADATA_SERVICE_ENDPOINT_MODE` environment variable.
    pub ec2_metadata_service_endpoint_mode: pulumi_gestalt_rust::Output<Option<String>>,
    pub endpoints: pulumi_gestalt_rust::Output<
        Option<Vec<super::types::ProviderEndpoint>>,
    >,
    pub forbidden_account_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    /// URL of a proxy to use for HTTP requests when accessing the AWS API. Can also be set using the `HTTP_PROXY` or
    /// `http_proxy` environment variables.
    pub http_proxy: pulumi_gestalt_rust::Output<Option<String>>,
    /// URL of a proxy to use for HTTPS requests when accessing the AWS API. Can also be set using the `HTTPS_PROXY` or
    /// `https_proxy` environment variables.
    pub https_proxy: pulumi_gestalt_rust::Output<Option<String>>,
    /// Configuration block with settings to ignore resource tags across all resources.
    pub ignore_tags: pulumi_gestalt_rust::Output<
        Option<super::types::ProviderIgnoreTags>,
    >,
    /// Explicitly allow the provider to perform "insecure" SSL requests. If omitted, default value is `false`
    pub insecure: pulumi_gestalt_rust::Output<Option<bool>>,
    /// The maximum number of times an AWS API request is being executed. If the API request still fails, an error is thrown.
    pub max_retries: pulumi_gestalt_rust::Output<Option<i32>>,
    /// Comma-separated list of hosts that should not use HTTP or HTTPS proxies. Can also be set using the `NO_PROXY` or
    /// `no_proxy` environment variables.
    pub no_proxy: pulumi_gestalt_rust::Output<Option<String>>,
    /// The profile for API operations. If not set, the default profile created with `aws configure` will be used.
    pub profile: pulumi_gestalt_rust::Output<Option<String>>,
    /// The region where AWS operations will take place. Examples are us-east-1, us-west-2, etc.
    pub region: pulumi_gestalt_rust::Output<Option<String>>,
    /// Specifies how retries are attempted. Valid values are `standard` and `adaptive`. Can also be configured using the
    /// `AWS_RETRY_MODE` environment variable.
    pub retry_mode: pulumi_gestalt_rust::Output<Option<String>>,
    /// Specifies whether S3 API calls in the `us-east-1` region use the legacy global endpoint or a regional endpoint. Valid
    /// values are `legacy` or `regional`. Can also be configured using the `AWS_S3_US_EAST_1_REGIONAL_ENDPOINT` environment
    /// variable or the `s3_us_east_1_regional_endpoint` shared config file parameter
    pub s3_us_east1_regional_endpoint: pulumi_gestalt_rust::Output<Option<String>>,
    /// Set this to true to enable the request to use path-style addressing, i.e., https://s3.amazonaws.com/BUCKET/KEY. By
    /// default, the S3 client will use virtual hosted bucket addressing when possible (https://BUCKET.s3.amazonaws.com/KEY).
    /// Specific to the Amazon S3 service.
    pub s3_use_path_style: pulumi_gestalt_rust::Output<Option<bool>>,
    /// The secret key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
    pub secret_key: pulumi_gestalt_rust::Output<Option<String>>,
    /// List of paths to shared config files. If not set, defaults to [~/.aws/config].
    pub shared_config_files: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    /// List of paths to shared credentials files. If not set, defaults to [~/.aws/credentials].
    pub shared_credentials_files: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
    /// Skip the credentials validation via STS API. Used for AWS API implementations that do not have STS
    /// available/implemented.
    pub skip_credentials_validation: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Skip the AWS Metadata API check. Used for AWS API implementations that do not have a metadata api endpoint.
    pub skip_metadata_api_check: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Skip static validation of region name. Used by users of alternative AWS-like APIs or users w/ access to regions that are
    /// not public (yet).
    pub skip_region_validation: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Skip requesting the account ID. Used for AWS API implementations that do not have IAM/STS API and/or metadata API.
    pub skip_requesting_account_id: pulumi_gestalt_rust::Output<Option<bool>>,
    /// The region where AWS STS operations will take place. Examples are us-east-1 and us-west-2.
    pub sts_region: pulumi_gestalt_rust::Output<Option<String>>,
    /// session token. A session token is only required if you are using temporary security credentials.
    pub token: pulumi_gestalt_rust::Output<Option<String>>,
    /// The capacity of the AWS SDK's token bucket rate limiter.
    pub token_bucket_rate_limiter_capacity: pulumi_gestalt_rust::Output<Option<i32>>,
    /// Resolve an endpoint with DualStack capability
    pub use_dualstack_endpoint: pulumi_gestalt_rust::Output<Option<bool>>,
    /// Resolve an endpoint with FIPS capability
    pub use_fips_endpoint: pulumi_gestalt_rust::Output<Option<bool>>,
}
impl pulumi_gestalt_rust::Provider for ProviderResult {
    fn get_provider_id(&self) -> pulumi_gestalt_rust::Output<String> {
        self.provider_id.clone()
    }
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
) -> ProviderResult {
    create_with_options(context, name, args, None)
}
///
/// Registers a new resource with the given unique name and arguments
///
#[allow(non_snake_case, unused_imports, dead_code)]
pub fn create_with_options(
    context: &pulumi_gestalt_rust::Context,
    name: &str,
    args: ProviderArgs,
    options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
) -> ProviderResult {
    let access_key_binding = args.access_key.get_output(context);
    let allowed_account_ids_binding = args.allowed_account_ids.get_output(context);
    let assume_role_binding = args.assume_role.get_output(context);
    let assume_role_with_web_identity_binding = args
        .assume_role_with_web_identity
        .get_output(context);
    let custom_ca_bundle_binding = args.custom_ca_bundle.get_output(context);
    let default_tags_binding = args.default_tags.get_output(context);
    let ec2_metadata_service_endpoint_binding = args
        .ec2_metadata_service_endpoint
        .get_output(context);
    let ec2_metadata_service_endpoint_mode_binding = args
        .ec2_metadata_service_endpoint_mode
        .get_output(context);
    let endpoints_binding = args.endpoints.get_output(context);
    let forbidden_account_ids_binding = args.forbidden_account_ids.get_output(context);
    let http_proxy_binding = args.http_proxy.get_output(context);
    let https_proxy_binding = args.https_proxy.get_output(context);
    let ignore_tags_binding = args.ignore_tags.get_output(context);
    let insecure_binding = args.insecure.get_output(context);
    let max_retries_binding = args.max_retries.get_output(context);
    let no_proxy_binding = args.no_proxy.get_output(context);
    let profile_binding = args.profile.get_output(context);
    let region_binding = args.region.get_output(context);
    let retry_mode_binding = args.retry_mode.get_output(context);
    let s3_us_east1_regional_endpoint_binding = args
        .s3_us_east1_regional_endpoint
        .get_output(context);
    let s3_use_path_style_binding = args.s3_use_path_style.get_output(context);
    let secret_key_binding = args.secret_key.get_output(context);
    let shared_config_files_binding = args.shared_config_files.get_output(context);
    let shared_credentials_files_binding = args
        .shared_credentials_files
        .get_output(context);
    let skip_credentials_validation_binding = args
        .skip_credentials_validation
        .get_output(context);
    let skip_metadata_api_check_binding = args
        .skip_metadata_api_check
        .get_output(context);
    let skip_region_validation_binding = args.skip_region_validation.get_output(context);
    let skip_requesting_account_id_binding = args
        .skip_requesting_account_id
        .get_output(context);
    let sts_region_binding = args.sts_region.get_output(context);
    let token_binding = args.token.get_output(context);
    let token_bucket_rate_limiter_capacity_binding = args
        .token_bucket_rate_limiter_capacity
        .get_output(context);
    let use_dualstack_endpoint_binding = args.use_dualstack_endpoint.get_output(context);
    let use_fips_endpoint_binding = args.use_fips_endpoint.get_output(context);
    let request = pulumi_gestalt_rust::RegisterResourceRequest {
        type_: "pulumi:providers:aws".into(),
        name: name.to_string(),
        version: super::get_version(),
        object: &[
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "accessKey".into(),
                value: &access_key_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "allowedAccountIds".into(),
                value: &allowed_account_ids_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "assumeRole".into(),
                value: &assume_role_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "assumeRoleWithWebIdentity".into(),
                value: &assume_role_with_web_identity_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "customCaBundle".into(),
                value: &custom_ca_bundle_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "defaultTags".into(),
                value: &default_tags_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "ec2MetadataServiceEndpoint".into(),
                value: &ec2_metadata_service_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "ec2MetadataServiceEndpointMode".into(),
                value: &ec2_metadata_service_endpoint_mode_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "endpoints".into(),
                value: &endpoints_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "forbiddenAccountIds".into(),
                value: &forbidden_account_ids_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "httpProxy".into(),
                value: &http_proxy_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "httpsProxy".into(),
                value: &https_proxy_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "ignoreTags".into(),
                value: &ignore_tags_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "insecure".into(),
                value: &insecure_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "maxRetries".into(),
                value: &max_retries_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "noProxy".into(),
                value: &no_proxy_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "profile".into(),
                value: &profile_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "region".into(),
                value: &region_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "retryMode".into(),
                value: &retry_mode_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "s3UsEast1RegionalEndpoint".into(),
                value: &s3_us_east1_regional_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "s3UsePathStyle".into(),
                value: &s3_use_path_style_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "secretKey".into(),
                value: &secret_key_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "sharedConfigFiles".into(),
                value: &shared_config_files_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "sharedCredentialsFiles".into(),
                value: &shared_credentials_files_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "skipCredentialsValidation".into(),
                value: &skip_credentials_validation_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "skipMetadataApiCheck".into(),
                value: &skip_metadata_api_check_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "skipRegionValidation".into(),
                value: &skip_region_validation_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "skipRequestingAccountId".into(),
                value: &skip_requesting_account_id_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "stsRegion".into(),
                value: &sts_region_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "token".into(),
                value: &token_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "tokenBucketRateLimiterCapacity".into(),
                value: &token_bucket_rate_limiter_capacity_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useDualstackEndpoint".into(),
                value: &use_dualstack_endpoint_binding.drop_type(),
            },
            pulumi_gestalt_rust::ResourceRequestObjectField {
                name: "useFipsEndpoint".into(),
                value: &use_fips_endpoint_binding.drop_type(),
            },
        ],
        options,
    };
    let o = context.register_resource(request);
    ProviderResult {
        urn: o.get_urn(),
        id: o.get_id(),
        provider_id: o.get_provider_id(),
        access_key: o.get_field("accessKey"),
        allowed_account_ids: o.get_field("allowedAccountIds"),
        assume_role: o.get_field("assumeRole"),
        assume_role_with_web_identity: o.get_field("assumeRoleWithWebIdentity"),
        custom_ca_bundle: o.get_field("customCaBundle"),
        default_tags: o.get_field("defaultTags"),
        ec2_metadata_service_endpoint: o.get_field("ec2MetadataServiceEndpoint"),
        ec2_metadata_service_endpoint_mode: o
            .get_field("ec2MetadataServiceEndpointMode"),
        endpoints: o.get_field("endpoints"),
        forbidden_account_ids: o.get_field("forbiddenAccountIds"),
        http_proxy: o.get_field("httpProxy"),
        https_proxy: o.get_field("httpsProxy"),
        ignore_tags: o.get_field("ignoreTags"),
        insecure: o.get_field("insecure"),
        max_retries: o.get_field("maxRetries"),
        no_proxy: o.get_field("noProxy"),
        profile: o.get_field("profile"),
        region: o.get_field("region"),
        retry_mode: o.get_field("retryMode"),
        s3_us_east1_regional_endpoint: o.get_field("s3UsEast1RegionalEndpoint"),
        s3_use_path_style: o.get_field("s3UsePathStyle"),
        secret_key: o.get_field("secretKey"),
        shared_config_files: o.get_field("sharedConfigFiles"),
        shared_credentials_files: o.get_field("sharedCredentialsFiles"),
        skip_credentials_validation: o.get_field("skipCredentialsValidation"),
        skip_metadata_api_check: o.get_field("skipMetadataApiCheck"),
        skip_region_validation: o.get_field("skipRegionValidation"),
        skip_requesting_account_id: o.get_field("skipRequestingAccountId"),
        sts_region: o.get_field("stsRegion"),
        token: o.get_field("token"),
        token_bucket_rate_limiter_capacity: o
            .get_field("tokenBucketRateLimiterCapacity"),
        use_dualstack_endpoint: o.get_field("useDualstackEndpoint"),
        use_fips_endpoint: o.get_field("useFipsEndpoint"),
    }
}
