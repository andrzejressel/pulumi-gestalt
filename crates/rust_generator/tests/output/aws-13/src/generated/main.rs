pub mod macie {
    include!("resources/macie/custom_data_identifier.rs");
    include!("resources/macie/findings_filter.rs");
}
pub mod macie2 {
    include!("resources/macie2/account.rs");
    include!("resources/macie2/classification_export_configuration.rs");
    include!("resources/macie2/classification_job.rs");
    include!("resources/macie2/invitation_accepter.rs");
    include!("resources/macie2/member.rs");
    include!("resources/macie2/organization_admin_account.rs");
}
pub mod mediaconvert {
    include!("resources/mediaconvert/queue.rs");
}
pub mod medialive {
    include!("resources/medialive/channel.rs");
    include!("resources/medialive/input.rs");
    include!("resources/medialive/input_security_group.rs");
    include!("resources/medialive/multiplex.rs");
    include!("resources/medialive/multiplex_program.rs");
}
pub mod mediapackage {
    include!("resources/mediapackage/channel.rs");
}
pub mod mediastore {
    include!("resources/mediastore/container.rs");
    include!("resources/mediastore/container_policy.rs");
}
pub mod memorydb {
    include!("resources/memorydb/acl.rs");
    include!("resources/memorydb/cluster.rs");
    include!("resources/memorydb/multi_region_cluster.rs");
    include!("resources/memorydb/parameter_group.rs");
    include!("resources/memorydb/snapshot.rs");
    include!("resources/memorydb/subnet_group.rs");
    include!("resources/memorydb/user.rs");
}
pub mod mq {
    include!("resources/mq/broker.rs");
    include!("resources/mq/configuration.rs");
}
pub mod msk {
    include!("resources/msk/cluster.rs");
    include!("resources/msk/cluster_policy.rs");
    include!("resources/msk/configuration.rs");
    include!("resources/msk/replicator.rs");
    include!("resources/msk/scram_secret_association.rs");
    include!("resources/msk/serverless_cluster.rs");
    include!("resources/msk/single_scram_secret_association.rs");
    include!("resources/msk/vpc_connection.rs");
}
pub mod mskconnect {
    include!("resources/mskconnect/connector.rs");
    include!("resources/mskconnect/custom_plugin.rs");
    include!("resources/mskconnect/worker_configuration.rs");
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
    pub mod mediaconvert {
        include!("functions/mediaconvert/get_queue.rs");
    }
    pub mod medialive {
        include!("functions/medialive/get_input.rs");
    }
    pub mod memorydb {
        include!("functions/memorydb/get_acl.rs");
        include!("functions/memorydb/get_cluster.rs");
        include!("functions/memorydb/get_parameter_group.rs");
        include!("functions/memorydb/get_snapshot.rs");
        include!("functions/memorydb/get_subnet_group.rs");
        include!("functions/memorydb/get_user.rs");
    }
    pub mod mq {
        include!("functions/mq/get_broker.rs");
        include!("functions/mq/get_broker_engine_types.rs");
        include!("functions/mq/get_instance_type_offerings.rs");
    }
    pub mod msk {
        include!("functions/msk/get_bootstrap_brokers.rs");
        include!("functions/msk/get_broker_nodes.rs");
        include!("functions/msk/get_cluster.rs");
        include!("functions/msk/get_configuration.rs");
        include!("functions/msk/get_kafka_version.rs");
        include!("functions/msk/get_vpc_connection.rs");
    }
    pub mod mskconnect {
        include!("functions/mskconnect/get_connector.rs");
        include!("functions/mskconnect/get_custom_plugin.rs");
        include!("functions/mskconnect/get_worker_configuration.rs");
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
    pub mod macie {
        include!("types/macie/findings_filter_finding_criteria.rs");
        include!("types/macie/findings_filter_finding_criteria_criterion.rs");
    }
    pub mod macie2 {
        include!("types/macie2/classification_export_configuration_s_3_destination.rs");
        include!("types/macie2/classification_job_s_3_job_definition.rs");
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_excludes.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_excludes_and.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_excludes_and_simple_criterion.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_excludes_and_tag_criterion.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_excludes_and_tag_criterion_tag_value.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_includes.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_includes_and.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_includes_and_simple_criterion.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_includes_and_tag_criterion.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_criteria_includes_and_tag_criterion_tag_value.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_bucket_definition.rs"
        );
        include!("types/macie2/classification_job_s_3_job_definition_scoping.rs");
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_excludes.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_excludes_and.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_excludes_and_simple_scope_term.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_excludes_and_tag_scope_term.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_excludes_and_tag_scope_term_tag_value.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_includes.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_includes_and.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_includes_and_simple_scope_term.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_includes_and_tag_scope_term.rs"
        );
        include!(
            "types/macie2/classification_job_s_3_job_definition_scoping_includes_and_tag_scope_term_tag_value.rs"
        );
        include!("types/macie2/classification_job_schedule_frequency.rs");
        include!("types/macie2/classification_job_user_paused_detail.rs");
    }
    pub mod mediaconvert {
        include!("types/mediaconvert/queue_reservation_plan_settings.rs");
    }
    pub mod medialive {
        include!("types/medialive/channel_cdi_input_specification.rs");
        include!("types/medialive/channel_destination.rs");
        include!("types/medialive/channel_destination_media_package_setting.rs");
        include!("types/medialive/channel_destination_multiplex_settings.rs");
        include!("types/medialive/channel_destination_setting.rs");
        include!("types/medialive/channel_encoder_settings.rs");
        include!("types/medialive/channel_encoder_settings_audio_description.rs");
        include!(
            "types/medialive/channel_encoder_settings_audio_description_audio_normalization_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_audio_watermark_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_audio_watermark_settings_nielsen_watermarks_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_audio_watermark_settings_nielsen_watermarks_settings_nielsen_cbet_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_audio_watermark_settings_nielsen_watermarks_settings_nielsen_naes_ii_nw_setting.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_aac_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_ac_3_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_eac_3_atmos_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_eac_3_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_mp_2_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_pass_through_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_codec_settings_wav_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_remix_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_remix_settings_channel_mapping.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_audio_description_remix_settings_channel_mapping_input_channel_level.rs"
        );
        include!("types/medialive/channel_encoder_settings_avail_blanking.rs");
        include!(
            "types/medialive/channel_encoder_settings_avail_blanking_avail_blanking_image.rs"
        );
        include!("types/medialive/channel_encoder_settings_caption_description.rs");
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_arib_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_burn_in_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_burn_in_destination_settings_font.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_dvb_sub_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_dvb_sub_destination_settings_font.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_ebu_tt_d_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_embedded_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_embedded_plus_scte_20_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_rtmp_caption_info_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_scte_20_plus_embedded_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_scte_27_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_smpte_tt_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_teletext_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_ttml_destination_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_caption_description_destination_settings_webvtt_destination_settings.rs"
        );
        include!("types/medialive/channel_encoder_settings_global_configuration.rs");
        include!(
            "types/medialive/channel_encoder_settings_global_configuration_input_loss_behavior.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_global_configuration_input_loss_behavior_input_loss_image_slate.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_motion_graphics_configuration.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_motion_graphics_configuration_motion_graphics_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_motion_graphics_configuration_motion_graphics_settings_html_motion_graphics_settings.rs"
        );
        include!("types/medialive/channel_encoder_settings_nielsen_configuration.rs");
        include!("types/medialive/channel_encoder_settings_output_group.rs");
        include!("types/medialive/channel_encoder_settings_output_group_output.rs");
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_archive_group_setting.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_archive_group_setting_archive_cdn_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_archive_group_setting_archive_cdn_settings_archive_s_3_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_archive_group_setting_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_frame_capture_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_frame_capture_group_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_frame_capture_group_settings_frame_capture_cdn_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_frame_capture_group_settings_frame_capture_cdn_settings_frame_capture_s_3_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_caption_language_mapping.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting_hls_akamai_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting_hls_basic_put_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting_hls_media_store_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting_hls_s_3_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_hls_cdn_setting_hls_webdav_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_key_provider_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_key_provider_settings_static_key_setting.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_hls_group_settings_key_provider_settings_static_key_setting_key_provider_server.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_media_package_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_media_package_group_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_ms_smooth_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_ms_smooth_group_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_multiplex_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_rtmp_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_group_settings_udp_group_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings_m_2_ts_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings_m_2_ts_settings_dvb_nit_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings_m_2_ts_settings_dvb_sdt_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings_m_2_ts_settings_dvb_tdt_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_archive_output_settings_container_settings_raw_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_frame_capture_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_audio_only_hls_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_audio_only_hls_settings_audio_only_image.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_fmp_4_hls_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_frame_capture_hls_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_standard_hls_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_hls_output_settings_hls_settings_standard_hls_settings_m_3_u_8_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_media_package_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_ms_smooth_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_multiplex_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_multiplex_output_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_rtmp_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_rtmp_output_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_container_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_container_settings_m_2_ts_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_container_settings_m_2_ts_settings_dvb_nit_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_container_settings_m_2_ts_settings_dvb_sdt_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_container_settings_m_2_ts_settings_dvb_tdt_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_destination.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_output_group_output_output_settings_udp_output_settings_fec_output_settings.rs"
        );
        include!("types/medialive/channel_encoder_settings_timecode_config.rs");
        include!("types/medialive/channel_encoder_settings_video_description.rs");
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_frame_capture_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_264_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_264_settings_filter_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_264_settings_filter_settings_temporal_filter_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings_color_space_passthrough_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings_dolby_vision_81_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings_hdr_10_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings_rec_601_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_color_space_settings_rec_709_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_filter_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_filter_settings_temporal_filter_settings.rs"
        );
        include!(
            "types/medialive/channel_encoder_settings_video_description_codec_settings_h_265_settings_timecode_burnin_settings.rs"
        );
        include!("types/medialive/channel_input_attachment.rs");
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings_failover_condition.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings_failover_condition_failover_condition_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings_failover_condition_failover_condition_settings_audio_silence_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings_failover_condition_failover_condition_settings_input_loss_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_automatic_input_failover_settings_failover_condition_failover_condition_settings_video_black_settings.rs"
        );
        include!("types/medialive/channel_input_attachment_input_settings.rs");
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_hls_rendition_selection.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_language_selection.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_pid_selection.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_track_selection.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_track_selection_dolby_e_decode.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_audio_selector_selector_settings_audio_track_selection_track.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_ancillary_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_arib_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_dvb_sub_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_embedded_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_scte_20_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_scte_27_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_teletext_source_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_caption_selector_selector_settings_teletext_source_settings_output_rectangle.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_network_input_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_network_input_settings_hls_input_settings.rs"
        );
        include!(
            "types/medialive/channel_input_attachment_input_settings_video_selector.rs"
        );
        include!("types/medialive/channel_input_specification.rs");
        include!("types/medialive/channel_maintenance.rs");
        include!("types/medialive/channel_vpc.rs");
        include!("types/medialive/input_destination.rs");
        include!("types/medialive/input_input_device.rs");
        include!("types/medialive/input_media_connect_flow.rs");
        include!("types/medialive/input_security_group_whitelist_rule.rs");
        include!("types/medialive/input_source.rs");
        include!("types/medialive/input_vpc.rs");
        include!("types/medialive/multiplex_multiplex_settings.rs");
        include!("types/medialive/multiplex_program_multiplex_program_settings.rs");
        include!(
            "types/medialive/multiplex_program_multiplex_program_settings_service_descriptor.rs"
        );
        include!(
            "types/medialive/multiplex_program_multiplex_program_settings_video_settings.rs"
        );
        include!(
            "types/medialive/multiplex_program_multiplex_program_settings_video_settings_statmux_settings.rs"
        );
        include!("types/medialive/get_input_destination.rs");
        include!("types/medialive/get_input_destination_vpc.rs");
        include!("types/medialive/get_input_input_device.rs");
        include!("types/medialive/get_input_media_connect_flow.rs");
        include!("types/medialive/get_input_source.rs");
    }
    pub mod mediapackage {
        include!("types/mediapackage/channel_hls_ingest.rs");
        include!("types/mediapackage/channel_hls_ingest_ingest_endpoint.rs");
    }
    pub mod memorydb {
        include!("types/memorydb/cluster_cluster_endpoint.rs");
        include!("types/memorydb/cluster_shard.rs");
        include!("types/memorydb/cluster_shard_node.rs");
        include!("types/memorydb/cluster_shard_node_endpoint.rs");
        include!("types/memorydb/multi_region_cluster_timeouts.rs");
        include!("types/memorydb/parameter_group_parameter.rs");
        include!("types/memorydb/snapshot_cluster_configuration.rs");
        include!("types/memorydb/user_authentication_mode.rs");
        include!("types/memorydb/get_cluster_cluster_endpoint.rs");
        include!("types/memorydb/get_cluster_shard.rs");
        include!("types/memorydb/get_cluster_shard_node.rs");
        include!("types/memorydb/get_cluster_shard_node_endpoint.rs");
        include!("types/memorydb/get_parameter_group_parameter.rs");
        include!("types/memorydb/get_snapshot_cluster_configuration.rs");
        include!("types/memorydb/get_user_authentication_mode.rs");
    }
    pub mod mq {
        include!("types/mq/broker_configuration.rs");
        include!("types/mq/broker_encryption_options.rs");
        include!("types/mq/broker_instance.rs");
        include!("types/mq/broker_ldap_server_metadata.rs");
        include!("types/mq/broker_logs.rs");
        include!("types/mq/broker_maintenance_window_start_time.rs");
        include!("types/mq/broker_user.rs");
        include!("types/mq/get_broker_configuration.rs");
        include!("types/mq/get_broker_encryption_option.rs");
        include!("types/mq/get_broker_engine_types_broker_engine_type.rs");
        include!(
            "types/mq/get_broker_engine_types_broker_engine_type_engine_version.rs"
        );
        include!("types/mq/get_broker_instance.rs");
        include!("types/mq/get_broker_ldap_server_metadata.rs");
        include!("types/mq/get_broker_logs.rs");
        include!("types/mq/get_broker_maintenance_window_start_time.rs");
        include!("types/mq/get_broker_user.rs");
        include!("types/mq/get_instance_type_offerings_broker_instance_option.rs");
        include!(
            "types/mq/get_instance_type_offerings_broker_instance_option_availability_zone.rs"
        );
    }
    pub mod msk {
        include!("types/msk/cluster_broker_node_group_info.rs");
        include!("types/msk/cluster_broker_node_group_info_connectivity_info.rs");
        include!(
            "types/msk/cluster_broker_node_group_info_connectivity_info_public_access.rs"
        );
        include!(
            "types/msk/cluster_broker_node_group_info_connectivity_info_vpc_connectivity.rs"
        );
        include!(
            "types/msk/cluster_broker_node_group_info_connectivity_info_vpc_connectivity_client_authentication.rs"
        );
        include!(
            "types/msk/cluster_broker_node_group_info_connectivity_info_vpc_connectivity_client_authentication_sasl.rs"
        );
        include!("types/msk/cluster_broker_node_group_info_storage_info.rs");
        include!(
            "types/msk/cluster_broker_node_group_info_storage_info_ebs_storage_info.rs"
        );
        include!(
            "types/msk/cluster_broker_node_group_info_storage_info_ebs_storage_info_provisioned_throughput.rs"
        );
        include!("types/msk/cluster_client_authentication.rs");
        include!("types/msk/cluster_client_authentication_sasl.rs");
        include!("types/msk/cluster_client_authentication_tls.rs");
        include!("types/msk/cluster_configuration_info.rs");
        include!("types/msk/cluster_encryption_info.rs");
        include!("types/msk/cluster_encryption_info_encryption_in_transit.rs");
        include!("types/msk/cluster_logging_info.rs");
        include!("types/msk/cluster_logging_info_broker_logs.rs");
        include!("types/msk/cluster_logging_info_broker_logs_cloudwatch_logs.rs");
        include!("types/msk/cluster_logging_info_broker_logs_firehose.rs");
        include!("types/msk/cluster_logging_info_broker_logs_s_3.rs");
        include!("types/msk/cluster_open_monitoring.rs");
        include!("types/msk/cluster_open_monitoring_prometheus.rs");
        include!("types/msk/cluster_open_monitoring_prometheus_jmx_exporter.rs");
        include!("types/msk/cluster_open_monitoring_prometheus_node_exporter.rs");
        include!("types/msk/replicator_kafka_cluster.rs");
        include!("types/msk/replicator_kafka_cluster_amazon_msk_cluster.rs");
        include!("types/msk/replicator_kafka_cluster_vpc_config.rs");
        include!("types/msk/replicator_replication_info_list.rs");
        include!(
            "types/msk/replicator_replication_info_list_consumer_group_replication.rs"
        );
        include!("types/msk/replicator_replication_info_list_topic_replication.rs");
        include!(
            "types/msk/replicator_replication_info_list_topic_replication_starting_position.rs"
        );
        include!(
            "types/msk/replicator_replication_info_list_topic_replication_topic_name_configuration.rs"
        );
        include!("types/msk/serverless_cluster_client_authentication.rs");
        include!("types/msk/serverless_cluster_client_authentication_sasl.rs");
        include!("types/msk/serverless_cluster_client_authentication_sasl_iam.rs");
        include!("types/msk/serverless_cluster_vpc_config.rs");
        include!("types/msk/get_broker_nodes_node_info_list.rs");
        include!("types/msk/get_cluster_broker_node_group_info.rs");
        include!("types/msk/get_cluster_broker_node_group_info_connectivity_info.rs");
        include!(
            "types/msk/get_cluster_broker_node_group_info_connectivity_info_public_access.rs"
        );
        include!(
            "types/msk/get_cluster_broker_node_group_info_connectivity_info_vpc_connectivity.rs"
        );
        include!(
            "types/msk/get_cluster_broker_node_group_info_connectivity_info_vpc_connectivity_client_authentication.rs"
        );
        include!(
            "types/msk/get_cluster_broker_node_group_info_connectivity_info_vpc_connectivity_client_authentication_sasl.rs"
        );
        include!("types/msk/get_cluster_broker_node_group_info_storage_info.rs");
        include!(
            "types/msk/get_cluster_broker_node_group_info_storage_info_ebs_storage_info.rs"
        );
        include!(
            "types/msk/get_cluster_broker_node_group_info_storage_info_ebs_storage_info_provisioned_throughput.rs"
        );
    }
    pub mod mskconnect {
        include!("types/mskconnect/connector_capacity.rs");
        include!("types/mskconnect/connector_capacity_autoscaling.rs");
        include!("types/mskconnect/connector_capacity_autoscaling_scale_in_policy.rs");
        include!("types/mskconnect/connector_capacity_autoscaling_scale_out_policy.rs");
        include!("types/mskconnect/connector_capacity_provisioned_capacity.rs");
        include!("types/mskconnect/connector_kafka_cluster.rs");
        include!("types/mskconnect/connector_kafka_cluster_apache_kafka_cluster.rs");
        include!("types/mskconnect/connector_kafka_cluster_apache_kafka_cluster_vpc.rs");
        include!("types/mskconnect/connector_kafka_cluster_client_authentication.rs");
        include!("types/mskconnect/connector_kafka_cluster_encryption_in_transit.rs");
        include!("types/mskconnect/connector_log_delivery.rs");
        include!("types/mskconnect/connector_log_delivery_worker_log_delivery.rs");
        include!(
            "types/mskconnect/connector_log_delivery_worker_log_delivery_cloudwatch_logs.rs"
        );
        include!(
            "types/mskconnect/connector_log_delivery_worker_log_delivery_firehose.rs"
        );
        include!("types/mskconnect/connector_log_delivery_worker_log_delivery_s_3.rs");
        include!("types/mskconnect/connector_plugin.rs");
        include!("types/mskconnect/connector_plugin_custom_plugin.rs");
        include!("types/mskconnect/connector_worker_configuration.rs");
        include!("types/mskconnect/custom_plugin_location.rs");
        include!("types/mskconnect/custom_plugin_location_s_3.rs");
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
