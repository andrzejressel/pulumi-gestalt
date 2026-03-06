pub mod vpc {
    include!("resources/vpc/endpoint_private_dns.rs");
    include!("resources/vpc/endpoint_service_private_dns_verification.rs");
    include!("resources/vpc/security_group_egress_rule.rs");
    include!("resources/vpc/security_group_ingress_rule.rs");
    include!("resources/vpc/security_group_vpc_association.rs");
}
pub mod vpclattice {
    include!("resources/vpclattice/access_log_subscription.rs");
    include!("resources/vpclattice/auth_policy.rs");
    include!("resources/vpclattice/listener.rs");
    include!("resources/vpclattice/listener_rule.rs");
    include!("resources/vpclattice/resource_policy.rs");
    include!("resources/vpclattice/service.rs");
    include!("resources/vpclattice/service_network.rs");
    include!("resources/vpclattice/service_network_service_association.rs");
    include!("resources/vpclattice/service_network_vpc_association.rs");
    include!("resources/vpclattice/target_group.rs");
    include!("resources/vpclattice/target_group_attachment.rs");
}
pub mod waf {
    include!("resources/waf/byte_match_set.rs");
    include!("resources/waf/geo_match_set.rs");
    include!("resources/waf/ip_set.rs");
    include!("resources/waf/rate_based_rule.rs");
    include!("resources/waf/regex_match_set.rs");
    include!("resources/waf/regex_pattern_set.rs");
    include!("resources/waf/rule.rs");
    include!("resources/waf/rule_group.rs");
    include!("resources/waf/size_constraint_set.rs");
    include!("resources/waf/sql_injection_match_set.rs");
    include!("resources/waf/web_acl.rs");
    include!("resources/waf/xss_match_set.rs");
}
pub mod wafregional {
    include!("resources/wafregional/byte_match_set.rs");
    include!("resources/wafregional/geo_match_set.rs");
    include!("resources/wafregional/ip_set.rs");
    include!("resources/wafregional/rate_based_rule.rs");
    include!("resources/wafregional/regex_match_set.rs");
    include!("resources/wafregional/regex_pattern_set.rs");
    include!("resources/wafregional/rule.rs");
    include!("resources/wafregional/rule_group.rs");
    include!("resources/wafregional/size_constraint_set.rs");
    include!("resources/wafregional/sql_injection_match_set.rs");
    include!("resources/wafregional/web_acl.rs");
    include!("resources/wafregional/web_acl_association.rs");
    include!("resources/wafregional/xss_match_set.rs");
}
pub mod wafv2 {
    include!("resources/wafv2/ip_set.rs");
    include!("resources/wafv2/regex_pattern_set.rs");
    include!("resources/wafv2/rule_group.rs");
    include!("resources/wafv2/web_acl.rs");
    include!("resources/wafv2/web_acl_association.rs");
    include!("resources/wafv2/web_acl_logging_configuration.rs");
}
pub mod worklink {
    include!("resources/worklink/fleet.rs");
    include!("resources/worklink/website_certificate_authority_association.rs");
}
pub mod workspaces {
    include!("resources/workspaces/connection_alias.rs");
    include!("resources/workspaces/directory.rs");
    include!("resources/workspaces/ip_group.rs");
    include!("resources/workspaces/workspace.rs");
}
pub mod xray {
    include!("resources/xray/encryption_config.rs");
    include!("resources/xray/group.rs");
    include!("resources/xray/sampling_rule.rs");
}
/// The provider type for the aws package. By default, resources use package-wide configuration
/// settings, however an explicit `Provider` instance may be created and passed during resource
/// construction to achieve fine-grained programmatic control over provider settings. See the
/// [documentation](https://www.pulumi.com/docs/reference/programming-model/#providers) for more information.
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod provider {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProviderArgs {
        /// The access key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
        #[builder(into, default)]
        pub access_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub allowed_account_ids: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        #[builder(into, default)]
        pub assume_role: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ProviderAssumeRole>,
        >,
        #[builder(into, default)]
        pub assume_role_with_web_identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ProviderAssumeRoleWithWebIdentity>,
        >,
        /// File containing custom root and intermediate certificates. Can also be configured using the `AWS_CA_BUNDLE` environment
        /// variable. (Setting `ca_bundle` in the shared config file is not supported.)
        #[builder(into, default)]
        pub custom_ca_bundle: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with settings to default resource tags across all resources.
        #[builder(into, default)]
        pub default_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ProviderDefaultTags>,
        >,
        /// Address of the EC2 metadata service endpoint to use. Can also be configured using the
        /// `AWS_EC2_METADATA_SERVICE_ENDPOINT` environment variable.
        #[builder(into, default)]
        pub ec2_metadata_service_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Protocol to use with EC2 metadata service endpoint.Valid values are `IPv4` and `IPv6`. Can also be configured using the
        /// `AWS_EC2_METADATA_SERVICE_ENDPOINT_MODE` environment variable.
        #[builder(into, default)]
        pub ec2_metadata_service_endpoint_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        #[builder(into, default)]
        pub endpoints: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::ProviderEndpoint>>,
        >,
        #[builder(into, default)]
        pub forbidden_account_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// URL of a proxy to use for HTTP requests when accessing the AWS API. Can also be set using the `HTTP_PROXY` or
        /// `http_proxy` environment variables.
        #[builder(into, default)]
        pub http_proxy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// URL of a proxy to use for HTTPS requests when accessing the AWS API. Can also be set using the `HTTPS_PROXY` or
        /// `https_proxy` environment variables.
        #[builder(into, default)]
        pub https_proxy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Configuration block with settings to ignore resource tags across all resources.
        #[builder(into, default)]
        pub ignore_tags: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::ProviderIgnoreTags>,
        >,
        /// Explicitly allow the provider to perform "insecure" SSL requests. If omitted, default value is `false`
        #[builder(into, default)]
        pub insecure: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The maximum number of times an AWS API request is being executed. If the API request still fails, an error is thrown.
        #[builder(into, default)]
        pub max_retries: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Comma-separated list of hosts that should not use HTTP or HTTPS proxies. Can also be set using the `NO_PROXY` or
        /// `no_proxy` environment variables.
        #[builder(into, default)]
        pub no_proxy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The profile for API operations. If not set, the default profile created with `aws configure` will be used.
        #[builder(into, default)]
        pub profile: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The region where AWS operations will take place. Examples are us-east-1, us-west-2, etc.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies how retries are attempted. Valid values are `standard` and `adaptive`. Can also be configured using the
        /// `AWS_RETRY_MODE` environment variable.
        #[builder(into, default)]
        pub retry_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether S3 API calls in the `us-east-1` region use the legacy global endpoint or a regional endpoint. Valid
        /// values are `legacy` or `regional`. Can also be configured using the `AWS_S3_US_EAST_1_REGIONAL_ENDPOINT` environment
        /// variable or the `s3_us_east_1_regional_endpoint` shared config file parameter
        #[builder(into, default)]
        pub s3_us_east1_regional_endpoint: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Set this to true to enable the request to use path-style addressing, i.e., https://s3.amazonaws.com/BUCKET/KEY. By
        /// default, the S3 client will use virtual hosted bucket addressing when possible (https://BUCKET.s3.amazonaws.com/KEY).
        /// Specific to the Amazon S3 service.
        #[builder(into, default)]
        pub s3_use_path_style: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The secret key for API operations. You can retrieve this from the 'Security & Credentials' section of the AWS console.
        #[builder(into, default)]
        pub secret_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of paths to shared config files. If not set, defaults to [~/.aws/config].
        #[builder(into, default)]
        pub shared_config_files: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of paths to shared credentials files. If not set, defaults to [~/.aws/credentials].
        #[builder(into, default)]
        pub shared_credentials_files: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Skip the credentials validation via STS API. Used for AWS API implementations that do not have STS
        /// available/implemented.
        #[builder(into, default)]
        pub skip_credentials_validation: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Skip the AWS Metadata API check. Used for AWS API implementations that do not have a metadata api endpoint.
        #[builder(into, default)]
        pub skip_metadata_api_check: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Skip static validation of region name. Used by users of alternative AWS-like APIs or users w/ access to regions that are
        /// not public (yet).
        #[builder(into, default)]
        pub skip_region_validation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Skip requesting the account ID. Used for AWS API implementations that do not have IAM/STS API and/or metadata API.
        #[builder(into, default)]
        pub skip_requesting_account_id: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The region where AWS STS operations will take place. Examples are us-east-1 and us-west-2.
        #[builder(into, default)]
        pub sts_region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// session token. A session token is only required if you are using temporary security credentials.
        #[builder(into, default)]
        pub token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The capacity of the AWS SDK's token bucket rate limiter.
        #[builder(into, default)]
        pub token_bucket_rate_limiter_capacity: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// Resolve an endpoint with DualStack capability
        #[builder(into, default)]
        pub use_dualstack_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Resolve an endpoint with FIPS capability
        #[builder(into, default)]
        pub use_fips_endpoint: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ProviderResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
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
        pub ec2_metadata_service_endpoint_mode: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
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
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ProviderArgs,
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
        let forbidden_account_ids_binding = args
            .forbidden_account_ids
            .get_output(context);
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
        let skip_region_validation_binding = args
            .skip_region_validation
            .get_output(context);
        let skip_requesting_account_id_binding = args
            .skip_requesting_account_id
            .get_output(context);
        let sts_region_binding = args.sts_region.get_output(context);
        let token_binding = args.token.get_output(context);
        let token_bucket_rate_limiter_capacity_binding = args
            .token_bucket_rate_limiter_capacity
            .get_output(context);
        let use_dualstack_endpoint_binding = args
            .use_dualstack_endpoint
            .get_output(context);
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
        };
        let o = context.register_resource(request);
        ProviderResult {
            id: o.get_field("id"),
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
}
pub mod functions {
    pub mod vpc {
        include!("functions/vpc/get_security_group_rule.rs");
        include!("functions/vpc/get_security_group_rules.rs");
    }
    pub mod vpclattice {
        include!("functions/vpclattice/get_auth_policy.rs");
        include!("functions/vpclattice/get_listener.rs");
        include!("functions/vpclattice/get_resource_policy.rs");
        include!("functions/vpclattice/get_service.rs");
        include!("functions/vpclattice/get_service_network.rs");
    }
    pub mod waf {
        include!("functions/waf/get_ipset.rs");
        include!("functions/waf/get_rate_based_rule.rs");
        include!("functions/waf/get_rule.rs");
        include!("functions/waf/get_subscribed_rule_group.rs");
        include!("functions/waf/get_web_acl.rs");
    }
    pub mod wafregional {
        include!("functions/wafregional/get_ipset.rs");
        include!("functions/wafregional/get_rate_based_mod.rs");
        include!("functions/wafregional/get_rule.rs");
        include!("functions/wafregional/get_subscribed_rule_group.rs");
        include!("functions/wafregional/get_web_acl.rs");
    }
    pub mod wafv2 {
        include!("functions/wafv2/get_ip_set.rs");
        include!("functions/wafv2/get_regex_pattern_set.rs");
        include!("functions/wafv2/get_rule_group.rs");
        include!("functions/wafv2/get_web_acl.rs");
    }
    pub mod workspaces {
        include!("functions/workspaces/get_bundle.rs");
        include!("functions/workspaces/get_directory.rs");
        include!("functions/workspaces/get_image.rs");
        include!("functions/workspaces/get_workspace.rs");
    }
    include!("functions/get_arn.rs");
    include!("functions/get_availability_zone.rs");
    include!("functions/get_availability_zones.rs");
    include!("functions/get_billing_service_account.rs");
    include!("functions/get_caller_identity.rs");
    include!("functions/get_default_tags.rs");
    include!("functions/get_ip_ranges.rs");
    include!("functions/get_partition.rs");
    include!("functions/get_region.rs");
    include!("functions/get_regions.rs");
    include!("functions/get_service.rs");
    include!("functions/get_service_principal.rs");
}
pub mod types {
    pub mod vpc {
        include!("types/vpc/endpoint_service_private_dns_verification_timeouts.rs");
        include!("types/vpc/security_group_vpc_association_timeouts.rs");
        include!("types/vpc/get_security_group_rule_filter.rs");
        include!("types/vpc/get_security_group_rules_filter.rs");
    }
    pub mod vpclattice {
        include!("types/vpclattice/listener_default_action.rs");
        include!("types/vpclattice/listener_default_action_fixed_response.rs");
        include!("types/vpclattice/listener_default_action_forward.rs");
        include!("types/vpclattice/listener_default_action_forward_target_group.rs");
        include!("types/vpclattice/listener_rule_action.rs");
        include!("types/vpclattice/listener_rule_action_fixed_response.rs");
        include!("types/vpclattice/listener_rule_action_forward.rs");
        include!("types/vpclattice/listener_rule_action_forward_target_group.rs");
        include!("types/vpclattice/listener_rule_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match_header_match.rs");
        include!(
            "types/vpclattice/listener_rule_match_http_match_header_match_match.rs"
        );
        include!("types/vpclattice/listener_rule_match_http_match_path_match.rs");
        include!("types/vpclattice/listener_rule_match_http_match_path_match_match.rs");
        include!("types/vpclattice/service_dns_entry.rs");
        include!("types/vpclattice/service_network_service_association_dns_entry.rs");
        include!("types/vpclattice/target_group_attachment_target.rs");
        include!("types/vpclattice/target_group_config.rs");
        include!("types/vpclattice/target_group_config_health_check.rs");
        include!("types/vpclattice/target_group_config_health_check_matcher.rs");
        include!("types/vpclattice/get_listener_default_action.rs");
        include!("types/vpclattice/get_listener_default_action_fixed_response.rs");
        include!("types/vpclattice/get_listener_default_action_forward.rs");
        include!("types/vpclattice/get_listener_default_action_forward_target_group.rs");
        include!("types/vpclattice/get_service_dns_entry.rs");
    }
    pub mod waf {
        include!("types/waf/byte_match_set_byte_match_tuple.rs");
        include!("types/waf/byte_match_set_byte_match_tuple_field_to_match.rs");
        include!("types/waf/geo_match_set_geo_match_constraint.rs");
        include!("types/waf/ip_set_ip_set_descriptor.rs");
        include!("types/waf/rate_based_rule_predicate.rs");
        include!("types/waf/regex_match_set_regex_match_tuple.rs");
        include!("types/waf/regex_match_set_regex_match_tuple_field_to_match.rs");
        include!("types/waf/rule_group_activated_rule.rs");
        include!("types/waf/rule_group_activated_rule_action.rs");
        include!("types/waf/rule_predicate.rs");
        include!("types/waf/size_constraint_set_size_constraint.rs");
        include!("types/waf/size_constraint_set_size_constraint_field_to_match.rs");
        include!("types/waf/sql_injection_match_set_sql_injection_match_tuple.rs");
        include!(
            "types/waf/sql_injection_match_set_sql_injection_match_tuple_field_to_match.rs"
        );
        include!("types/waf/web_acl_default_action.rs");
        include!("types/waf/web_acl_logging_configuration.rs");
        include!("types/waf/web_acl_logging_configuration_redacted_fields.rs");
        include!(
            "types/waf/web_acl_logging_configuration_redacted_fields_field_to_match.rs"
        );
        include!("types/waf/web_acl_rule.rs");
        include!("types/waf/web_acl_rule_action.rs");
        include!("types/waf/web_acl_rule_override_action.rs");
        include!("types/waf/xss_match_set_xss_match_tuple.rs");
        include!("types/waf/xss_match_set_xss_match_tuple_field_to_match.rs");
    }
    pub mod wafregional {
        include!("types/wafregional/byte_match_set_byte_match_tuple.rs");
        include!("types/wafregional/byte_match_set_byte_match_tuple_field_to_match.rs");
        include!("types/wafregional/geo_match_set_geo_match_constraint.rs");
        include!("types/wafregional/ip_set_ip_set_descriptor.rs");
        include!("types/wafregional/rate_based_rule_predicate.rs");
        include!("types/wafregional/regex_match_set_regex_match_tuple.rs");
        include!(
            "types/wafregional/regex_match_set_regex_match_tuple_field_to_match.rs"
        );
        include!("types/wafregional/rule_group_activated_rule.rs");
        include!("types/wafregional/rule_group_activated_rule_action.rs");
        include!("types/wafregional/rule_predicate.rs");
        include!("types/wafregional/size_constraint_set_size_constraint.rs");
        include!(
            "types/wafregional/size_constraint_set_size_constraint_field_to_match.rs"
        );
        include!(
            "types/wafregional/sql_injection_match_set_sql_injection_match_tuple.rs"
        );
        include!(
            "types/wafregional/sql_injection_match_set_sql_injection_match_tuple_field_to_match.rs"
        );
        include!("types/wafregional/web_acl_default_action.rs");
        include!("types/wafregional/web_acl_logging_configuration.rs");
        include!("types/wafregional/web_acl_logging_configuration_redacted_fields.rs");
        include!(
            "types/wafregional/web_acl_logging_configuration_redacted_fields_field_to_match.rs"
        );
        include!("types/wafregional/web_acl_rule.rs");
        include!("types/wafregional/web_acl_rule_action.rs");
        include!("types/wafregional/web_acl_rule_override_action.rs");
        include!("types/wafregional/xss_match_set_xss_match_tuple.rs");
        include!("types/wafregional/xss_match_set_xss_match_tuple_field_to_match.rs");
    }
    pub mod wafv2 {
        include!("types/wafv2/regex_pattern_set_regular_expression.rs");
        include!("types/wafv2/rule_group_custom_response_body.rs");
        include!("types/wafv2/rule_group_rule.rs");
        include!("types/wafv2/rule_group_rule_action.rs");
        include!("types/wafv2/rule_group_rule_action_allow.rs");
        include!("types/wafv2/rule_group_rule_action_allow_custom_request_handling.rs");
        include!(
            "types/wafv2/rule_group_rule_action_allow_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/rule_group_rule_action_block.rs");
        include!("types/wafv2/rule_group_rule_action_block_custom_response.rs");
        include!(
            "types/wafv2/rule_group_rule_action_block_custom_response_response_header.rs"
        );
        include!("types/wafv2/rule_group_rule_action_captcha.rs");
        include!(
            "types/wafv2/rule_group_rule_action_captcha_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_action_captcha_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/rule_group_rule_action_challenge.rs");
        include!(
            "types/wafv2/rule_group_rule_action_challenge_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_action_challenge_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/rule_group_rule_action_count.rs");
        include!("types/wafv2/rule_group_rule_action_count_custom_request_handling.rs");
        include!(
            "types/wafv2/rule_group_rule_action_count_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/rule_group_rule_captcha_config.rs");
        include!("types/wafv2/rule_group_rule_captcha_config_immunity_time_property.rs");
        include!("types/wafv2/rule_group_rule_rule_label.rs");
        include!("types/wafv2/rule_group_rule_statement.rs");
        include!("types/wafv2/rule_group_rule_statement_and_statement.rs");
        include!("types/wafv2/rule_group_rule_statement_byte_match_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_byte_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_geo_match_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_geo_match_statement_forwarded_ip_config.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_ip_set_reference_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_ip_set_reference_statement_ip_set_forwarded_ip_config.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_label_match_statement.rs");
        include!("types/wafv2/rule_group_rule_statement_not_statement.rs");
        include!("types/wafv2/rule_group_rule_statement_or_statement.rs");
        include!("types/wafv2/rule_group_rule_statement_rate_based_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_cookie.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_cookie_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_forwarded_ip.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_header_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_http_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_ip.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_label_namespace.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_query_argument_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_query_string_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_custom_key_uri_path_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_and_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_geo_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_geo_match_statement_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_ip_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_ip_set_reference_statement_ip_set_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_label_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_not_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_or_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_regex_match_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_regex_pattern_set_reference_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_size_constraint_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_size_constraint_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_sqli_match_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_sqli_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_statement_xss_match_statement.rs");
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/rule_group_rule_statement_xss_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/rule_group_rule_visibility_config.rs");
        include!("types/wafv2/rule_group_visibility_config.rs");
        include!("types/wafv2/web_acl_association_config.rs");
        include!("types/wafv2/web_acl_association_config_request_body.rs");
        include!("types/wafv2/web_acl_association_config_request_body_api_gateway.rs");
        include!(
            "types/wafv2/web_acl_association_config_request_body_app_runner_service.rs"
        );
        include!("types/wafv2/web_acl_association_config_request_body_cloudfront.rs");
        include!(
            "types/wafv2/web_acl_association_config_request_body_cognito_user_pool.rs"
        );
        include!(
            "types/wafv2/web_acl_association_config_request_body_verified_access_instance.rs"
        );
        include!("types/wafv2/web_acl_captcha_config.rs");
        include!("types/wafv2/web_acl_captcha_config_immunity_time_property.rs");
        include!("types/wafv2/web_acl_challenge_config.rs");
        include!("types/wafv2/web_acl_challenge_config_immunity_time_property.rs");
        include!("types/wafv2/web_acl_custom_response_body.rs");
        include!("types/wafv2/web_acl_default_action.rs");
        include!("types/wafv2/web_acl_default_action_allow.rs");
        include!("types/wafv2/web_acl_default_action_allow_custom_request_handling.rs");
        include!(
            "types/wafv2/web_acl_default_action_allow_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_default_action_block.rs");
        include!("types/wafv2/web_acl_default_action_block_custom_response.rs");
        include!(
            "types/wafv2/web_acl_default_action_block_custom_response_response_header.rs"
        );
        include!("types/wafv2/web_acl_logging_configuration_logging_filter.rs");
        include!("types/wafv2/web_acl_logging_configuration_logging_filter_filter.rs");
        include!(
            "types/wafv2/web_acl_logging_configuration_logging_filter_filter_condition.rs"
        );
        include!(
            "types/wafv2/web_acl_logging_configuration_logging_filter_filter_condition_action_condition.rs"
        );
        include!(
            "types/wafv2/web_acl_logging_configuration_logging_filter_filter_condition_label_name_condition.rs"
        );
        include!("types/wafv2/web_acl_logging_configuration_redacted_field.rs");
        include!("types/wafv2/web_acl_logging_configuration_redacted_field_method.rs");
        include!(
            "types/wafv2/web_acl_logging_configuration_redacted_field_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_logging_configuration_redacted_field_single_header.rs"
        );
        include!("types/wafv2/web_acl_logging_configuration_redacted_field_uri_path.rs");
        include!("types/wafv2/web_acl_rule.rs");
        include!("types/wafv2/web_acl_rule_action.rs");
        include!("types/wafv2/web_acl_rule_action_allow.rs");
        include!("types/wafv2/web_acl_rule_action_allow_custom_request_handling.rs");
        include!(
            "types/wafv2/web_acl_rule_action_allow_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_rule_action_block.rs");
        include!("types/wafv2/web_acl_rule_action_block_custom_response.rs");
        include!(
            "types/wafv2/web_acl_rule_action_block_custom_response_response_header.rs"
        );
        include!("types/wafv2/web_acl_rule_action_captcha.rs");
        include!("types/wafv2/web_acl_rule_action_captcha_custom_request_handling.rs");
        include!(
            "types/wafv2/web_acl_rule_action_captcha_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_rule_action_challenge.rs");
        include!("types/wafv2/web_acl_rule_action_challenge_custom_request_handling.rs");
        include!(
            "types/wafv2/web_acl_rule_action_challenge_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_rule_action_count.rs");
        include!("types/wafv2/web_acl_rule_action_count_custom_request_handling.rs");
        include!(
            "types/wafv2/web_acl_rule_action_count_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_rule_captcha_config.rs");
        include!("types/wafv2/web_acl_rule_captcha_config_immunity_time_property.rs");
        include!("types/wafv2/web_acl_rule_override_action.rs");
        include!("types/wafv2/web_acl_rule_override_action_count.rs");
        include!("types/wafv2/web_acl_rule_override_action_none.rs");
        include!("types/wafv2/web_acl_rule_rule_label.rs");
        include!("types/wafv2/web_acl_rule_statement.rs");
        include!("types/wafv2/web_acl_rule_statement_and_statement.rs");
        include!("types/wafv2/web_acl_rule_statement_byte_match_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_byte_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_geo_match_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_geo_match_statement_forwarded_ip_config.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_ip_set_reference_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_ip_set_reference_statement_ip_set_forwarded_ip_config.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_label_match_statement.rs");
        include!("types/wafv2/web_acl_rule_statement_managed_rule_group_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection_address_fields.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection_email_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection_password_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection_phone_number_fields.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_request_inspection_username_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_response_inspection.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_response_inspection_body_contains.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_response_inspection_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_response_inspection_json.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_acfp_rule_set_response_inspection_status_code.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_request_inspection.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_request_inspection_password_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_request_inspection_username_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_response_inspection.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_response_inspection_body_contains.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_response_inspection_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_response_inspection_json.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_atp_rule_set_response_inspection_status_code.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_aws_managed_rules_bot_control_rule_set.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_password_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_managed_rule_group_config_username_field.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_allow.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_allow_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_allow_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_block.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_block_custom_response.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_block_custom_response_response_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_captcha.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_captcha_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_captcha_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_challenge.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_challenge_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_challenge_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_count.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_count_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_rule_action_override_action_to_use_count_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_and_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_byte_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_geo_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_geo_match_statement_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_ip_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_ip_set_reference_statement_ip_set_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_label_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_not_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_or_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_regex_pattern_set_reference_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_size_constraint_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_sqli_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_managed_rule_group_statement_scope_down_statement_xss_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_not_statement.rs");
        include!("types/wafv2/web_acl_rule_statement_or_statement.rs");
        include!("types/wafv2/web_acl_rule_statement_rate_based_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_cookie.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_cookie_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_forwarded_ip.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_header_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_http_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_ip.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_label_namespace.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_query_argument_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_query_string_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_custom_key_uri_path_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_and_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_byte_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_geo_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_geo_match_statement_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_ip_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_ip_set_reference_statement_ip_set_forwarded_ip_config.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_label_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_not_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_or_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_regex_pattern_set_reference_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_size_constraint_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_sqli_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rate_based_statement_scope_down_statement_xss_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_regex_match_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_match_statement_text_transformation.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_regex_pattern_set_reference_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_rule_group_reference_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_allow.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_allow_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_allow_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_block.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_block_custom_response.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_block_custom_response_response_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_captcha.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_captcha_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_captcha_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_challenge.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_challenge_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_challenge_custom_request_handling_insert_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_count.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_count_custom_request_handling.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_rule_group_reference_statement_rule_action_override_action_to_use_count_custom_request_handling_insert_header.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_size_constraint_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_size_constraint_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_sqli_match_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_sqli_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_statement_xss_match_statement.rs");
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_all_query_arguments.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_cookies.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_cookies_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_cookies_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_header_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_header_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_header_order.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_ja_3_fingerprint.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_json_body.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_json_body_match_pattern.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_json_body_match_pattern_all.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_method.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_query_string.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_single_header.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_single_query_argument.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_field_to_match_uri_path.rs"
        );
        include!(
            "types/wafv2/web_acl_rule_statement_xss_match_statement_text_transformation.rs"
        );
        include!("types/wafv2/web_acl_rule_visibility_config.rs");
        include!("types/wafv2/web_acl_visibility_config.rs");
        include!("types/wafv2/get_regex_pattern_set_regular_expression.rs");
    }
    pub mod worklink {
        include!("types/worklink/fleet_identity_provider.rs");
        include!("types/worklink/fleet_network.rs");
    }
    pub mod workspaces {
        include!("types/workspaces/connection_alias_timeouts.rs");
        include!("types/workspaces/directory_saml_properties.rs");
        include!("types/workspaces/directory_self_service_permissions.rs");
        include!("types/workspaces/directory_workspace_access_properties.rs");
        include!("types/workspaces/directory_workspace_creation_properties.rs");
        include!("types/workspaces/ip_group_rule.rs");
        include!("types/workspaces/workspace_workspace_properties.rs");
        include!("types/workspaces/get_bundle_compute_type.rs");
        include!("types/workspaces/get_bundle_root_storage.rs");
        include!("types/workspaces/get_bundle_user_storage.rs");
        include!("types/workspaces/get_directory_saml_property.rs");
        include!("types/workspaces/get_directory_self_service_permission.rs");
        include!("types/workspaces/get_directory_workspace_access_property.rs");
        include!("types/workspaces/get_directory_workspace_creation_property.rs");
        include!("types/workspaces/get_workspace_workspace_property.rs");
    }
    pub mod xray {
        include!("types/xray/group_insights_configuration.rs");
    }
    include!("types/get_availability_zone_filter.rs");
    include!("types/get_availability_zones_filter.rs");
    include!("types/get_regions_filter.rs");
}
#[doc(hidden)]
pub mod constants {}
#[unsafe(link_section = "pulumi_gestalt_provider::aws")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_AWS: [u8; 45] = *b"{\"version\":\"6.66.2\",\"pluginDownloadURL\":null}";
pub(crate) fn get_version() -> String {
    "6.66.2".to_string()
}
