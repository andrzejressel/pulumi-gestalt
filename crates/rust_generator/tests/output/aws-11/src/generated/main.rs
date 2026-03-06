pub mod imagebuilder {
    include!("resources/imagebuilder/component.rs");
    include!("resources/imagebuilder/container_recipe.rs");
    include!("resources/imagebuilder/distribution_configuration.rs");
    include!("resources/imagebuilder/image.rs");
    include!("resources/imagebuilder/image_pipeline.rs");
    include!("resources/imagebuilder/image_recipe.rs");
    include!("resources/imagebuilder/infrastructure_configuration.rs");
    include!("resources/imagebuilder/lifecycle_policy.rs");
    include!("resources/imagebuilder/workflow.rs");
}
pub mod inspector {
    include!("resources/inspector/assessment_target.rs");
    include!("resources/inspector/assessment_template.rs");
    include!("resources/inspector/resource_group.rs");
}
pub mod inspector2 {
    include!("resources/inspector2/delegated_admin_account.rs");
    include!("resources/inspector2/enabler.rs");
    include!("resources/inspector2/member_association.rs");
    include!("resources/inspector2/organization_configuration.rs");
}
pub mod iot {
    include!("resources/iot/authorizer.rs");
    include!("resources/iot/billing_group.rs");
    include!("resources/iot/ca_certificate.rs");
    include!("resources/iot/certificate.rs");
    include!("resources/iot/domain_configuration.rs");
    include!("resources/iot/event_configurations.rs");
    include!("resources/iot/indexing_configuration.rs");
    include!("resources/iot/logging_options.rs");
    include!("resources/iot/policy.rs");
    include!("resources/iot/policy_attachment.rs");
    include!("resources/iot/provisioning_template.rs");
    include!("resources/iot/role_alias.rs");
    include!("resources/iot/thing.rs");
    include!("resources/iot/thing_group.rs");
    include!("resources/iot/thing_group_membership.rs");
    include!("resources/iot/thing_principal_attachment.rs");
    include!("resources/iot/thing_type.rs");
    include!("resources/iot/topic_rule.rs");
    include!("resources/iot/topic_rule_destination.rs");
}
pub mod ivs {
    include!("resources/ivs/channel.rs");
    include!("resources/ivs/playback_key_pair.rs");
    include!("resources/ivs/recording_configuration.rs");
}
pub mod ivschat {
    include!("resources/ivschat/logging_configuration.rs");
    include!("resources/ivschat/room.rs");
}
pub mod kendra {
    include!("resources/kendra/data_source.rs");
    include!("resources/kendra/experience.rs");
    include!("resources/kendra/faq.rs");
    include!("resources/kendra/index.rs");
    include!("resources/kendra/query_suggestions_block_list.rs");
    include!("resources/kendra/thesaurus.rs");
}
pub mod keyspaces {
    include!("resources/keyspaces/keyspace.rs");
    include!("resources/keyspaces/table.rs");
}
pub mod kinesis {
    include!("resources/kinesis/analytics_application.rs");
    include!("resources/kinesis/firehose_delivery_stream.rs");
    include!("resources/kinesis/resource_policy.rs");
    include!("resources/kinesis/stream.rs");
    include!("resources/kinesis/stream_consumer.rs");
    include!("resources/kinesis/video_stream.rs");
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
    pub mod imagebuilder {
        include!("functions/imagebuilder/get_component.rs");
        include!("functions/imagebuilder/get_components.rs");
        include!("functions/imagebuilder/get_container_recipe.rs");
        include!("functions/imagebuilder/get_container_recipes.rs");
        include!("functions/imagebuilder/get_distribution_configuration.rs");
        include!("functions/imagebuilder/get_distribution_configurations.rs");
        include!("functions/imagebuilder/get_image.rs");
        include!("functions/imagebuilder/get_image_pipeline.rs");
        include!("functions/imagebuilder/get_image_pipelines.rs");
        include!("functions/imagebuilder/get_image_recipe.rs");
        include!("functions/imagebuilder/get_image_recipes.rs");
        include!("functions/imagebuilder/get_infrastructure_configuration.rs");
        include!("functions/imagebuilder/get_infrastructure_configurations.rs");
    }
    pub mod inspector {
        include!("functions/inspector/get_rules_packages.rs");
    }
    pub mod iot {
        include!("functions/iot/get_endpoint.rs");
        include!("functions/iot/get_registration_code.rs");
    }
    pub mod ivs {
        include!("functions/ivs/get_stream_key.rs");
    }
    pub mod kendra {
        include!("functions/kendra/get_experience.rs");
        include!("functions/kendra/get_faq.rs");
        include!("functions/kendra/get_index.rs");
        include!("functions/kendra/get_query_suggestions_block_list.rs");
        include!("functions/kendra/get_thesaurus.rs");
    }
    pub mod kinesis {
        include!("functions/kinesis/get_firehose_delivery_stream.rs");
        include!("functions/kinesis/get_stream.rs");
        include!("functions/kinesis/get_stream_consumer.rs");
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
    pub mod imagebuilder {
        include!("types/imagebuilder/container_recipe_component.rs");
        include!("types/imagebuilder/container_recipe_component_parameter.rs");
        include!("types/imagebuilder/container_recipe_instance_configuration.rs");
        include!(
            "types/imagebuilder/container_recipe_instance_configuration_block_device_mapping.rs"
        );
        include!(
            "types/imagebuilder/container_recipe_instance_configuration_block_device_mapping_ebs.rs"
        );
        include!("types/imagebuilder/container_recipe_target_repository.rs");
        include!("types/imagebuilder/distribution_configuration_distribution.rs");
        include!(
            "types/imagebuilder/distribution_configuration_distribution_ami_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_ami_distribution_configuration_launch_permission.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_container_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_container_distribution_configuration_target_repository.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration_launch_template.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_fast_launch_configuration_snapshot_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_launch_template_configuration.rs"
        );
        include!(
            "types/imagebuilder/distribution_configuration_distribution_s_3_export_configuration.rs"
        );
        include!("types/imagebuilder/image_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/image_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/image_image_tests_configuration.rs");
        include!("types/imagebuilder/image_output_resource.rs");
        include!("types/imagebuilder/image_output_resource_ami.rs");
        include!("types/imagebuilder/image_output_resource_container.rs");
        include!("types/imagebuilder/image_pipeline_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/image_pipeline_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/image_pipeline_image_tests_configuration.rs");
        include!("types/imagebuilder/image_pipeline_schedule.rs");
        include!("types/imagebuilder/image_pipeline_workflow.rs");
        include!("types/imagebuilder/image_pipeline_workflow_parameter.rs");
        include!("types/imagebuilder/image_recipe_block_device_mapping.rs");
        include!("types/imagebuilder/image_recipe_block_device_mapping_ebs.rs");
        include!("types/imagebuilder/image_recipe_component.rs");
        include!("types/imagebuilder/image_recipe_component_parameter.rs");
        include!("types/imagebuilder/image_recipe_systems_manager_agent.rs");
        include!("types/imagebuilder/image_workflow.rs");
        include!("types/imagebuilder/image_workflow_parameter.rs");
        include!(
            "types/imagebuilder/infrastructure_configuration_instance_metadata_options.rs"
        );
        include!("types/imagebuilder/infrastructure_configuration_logging.rs");
        include!("types/imagebuilder/infrastructure_configuration_logging_s_3_logs.rs");
        include!("types/imagebuilder/lifecycle_policy_policy_detail.rs");
        include!("types/imagebuilder/lifecycle_policy_policy_detail_action.rs");
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_action_include_resources.rs"
        );
        include!("types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules.rs");
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules_amis.rs"
        );
        include!(
            "types/imagebuilder/lifecycle_policy_policy_detail_exclusion_rules_amis_last_launched.rs"
        );
        include!("types/imagebuilder/lifecycle_policy_policy_detail_filter.rs");
        include!("types/imagebuilder/lifecycle_policy_resource_selection.rs");
        include!("types/imagebuilder/lifecycle_policy_resource_selection_recipe.rs");
        include!("types/imagebuilder/get_components_filter.rs");
        include!("types/imagebuilder/get_container_recipe_component.rs");
        include!("types/imagebuilder/get_container_recipe_component_parameter.rs");
        include!("types/imagebuilder/get_container_recipe_instance_configuration.rs");
        include!(
            "types/imagebuilder/get_container_recipe_instance_configuration_block_device_mapping.rs"
        );
        include!(
            "types/imagebuilder/get_container_recipe_instance_configuration_block_device_mapping_eb.rs"
        );
        include!("types/imagebuilder/get_container_recipe_target_repository.rs");
        include!("types/imagebuilder/get_container_recipes_filter.rs");
        include!("types/imagebuilder/get_distribution_configuration_distribution.rs");
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_ami_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_ami_distribution_configuration_launch_permission.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_container_distribution_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_container_distribution_configuration_target_repository.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration_launch_template.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_fast_launch_configuration_snapshot_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_launch_template_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_distribution_configuration_distribution_s_3_export_configuration.rs"
        );
        include!("types/imagebuilder/get_distribution_configurations_filter.rs");
        include!("types/imagebuilder/get_image_image_scanning_configuration.rs");
        include!(
            "types/imagebuilder/get_image_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/get_image_image_tests_configuration.rs");
        include!("types/imagebuilder/get_image_output_resource.rs");
        include!("types/imagebuilder/get_image_output_resource_ami.rs");
        include!("types/imagebuilder/get_image_output_resource_container.rs");
        include!(
            "types/imagebuilder/get_image_pipeline_image_scanning_configuration.rs"
        );
        include!(
            "types/imagebuilder/get_image_pipeline_image_scanning_configuration_ecr_configuration.rs"
        );
        include!("types/imagebuilder/get_image_pipeline_image_tests_configuration.rs");
        include!("types/imagebuilder/get_image_pipeline_schedule.rs");
        include!("types/imagebuilder/get_image_pipelines_filter.rs");
        include!("types/imagebuilder/get_image_recipe_block_device_mapping.rs");
        include!("types/imagebuilder/get_image_recipe_block_device_mapping_eb.rs");
        include!("types/imagebuilder/get_image_recipe_component.rs");
        include!("types/imagebuilder/get_image_recipe_component_parameter.rs");
        include!("types/imagebuilder/get_image_recipes_filter.rs");
        include!(
            "types/imagebuilder/get_infrastructure_configuration_instance_metadata_option.rs"
        );
        include!("types/imagebuilder/get_infrastructure_configuration_logging.rs");
        include!(
            "types/imagebuilder/get_infrastructure_configuration_logging_s_3_log.rs"
        );
        include!("types/imagebuilder/get_infrastructure_configurations_filter.rs");
    }
    pub mod inspector {
        include!("types/inspector/assessment_template_event_subscription.rs");
    }
    pub mod inspector2 {
        include!("types/inspector2/organization_configuration_auto_enable.rs");
    }
    pub mod iot {
        include!("types/iot/billing_group_metadata.rs");
        include!("types/iot/billing_group_properties.rs");
        include!("types/iot/ca_certificate_registration_config.rs");
        include!("types/iot/ca_certificate_validity.rs");
        include!("types/iot/domain_configuration_authorizer_config.rs");
        include!("types/iot/domain_configuration_tls_config.rs");
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration_custom_field.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_group_indexing_configuration_managed_field.rs"
        );
        include!("types/iot/indexing_configuration_thing_indexing_configuration.rs");
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_custom_field.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_filter.rs"
        );
        include!(
            "types/iot/indexing_configuration_thing_indexing_configuration_managed_field.rs"
        );
        include!("types/iot/provisioning_template_pre_provisioning_hook.rs");
        include!("types/iot/thing_group_metadata.rs");
        include!("types/iot/thing_group_metadata_root_to_parent_group.rs");
        include!("types/iot/thing_group_properties.rs");
        include!("types/iot/thing_group_properties_attribute_payload.rs");
        include!("types/iot/thing_type_properties.rs");
        include!("types/iot/topic_rule_cloudwatch_alarm.rs");
        include!("types/iot/topic_rule_cloudwatch_log.rs");
        include!("types/iot/topic_rule_cloudwatch_metric.rs");
        include!("types/iot/topic_rule_destination_vpc_configuration.rs");
        include!("types/iot/topic_rule_dynamodb.rs");
        include!("types/iot/topic_rule_dynamodbv_2.rs");
        include!("types/iot/topic_rule_dynamodbv_2_put_item.rs");
        include!("types/iot/topic_rule_elasticsearch.rs");
        include!("types/iot/topic_rule_error_action.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_alarm.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_logs.rs");
        include!("types/iot/topic_rule_error_action_cloudwatch_metric.rs");
        include!("types/iot/topic_rule_error_action_dynamodb.rs");
        include!("types/iot/topic_rule_error_action_dynamodbv_2.rs");
        include!("types/iot/topic_rule_error_action_dynamodbv_2_put_item.rs");
        include!("types/iot/topic_rule_error_action_elasticsearch.rs");
        include!("types/iot/topic_rule_error_action_firehose.rs");
        include!("types/iot/topic_rule_error_action_http.rs");
        include!("types/iot/topic_rule_error_action_http_http_header.rs");
        include!("types/iot/topic_rule_error_action_iot_analytics.rs");
        include!("types/iot/topic_rule_error_action_iot_events.rs");
        include!("types/iot/topic_rule_error_action_kafka.rs");
        include!("types/iot/topic_rule_error_action_kafka_header.rs");
        include!("types/iot/topic_rule_error_action_kinesis.rs");
        include!("types/iot/topic_rule_error_action_lambda.rs");
        include!("types/iot/topic_rule_error_action_republish.rs");
        include!("types/iot/topic_rule_error_action_s_3.rs");
        include!("types/iot/topic_rule_error_action_sns.rs");
        include!("types/iot/topic_rule_error_action_sqs.rs");
        include!("types/iot/topic_rule_error_action_step_functions.rs");
        include!("types/iot/topic_rule_error_action_timestream.rs");
        include!("types/iot/topic_rule_error_action_timestream_dimension.rs");
        include!("types/iot/topic_rule_error_action_timestream_timestamp.rs");
        include!("types/iot/topic_rule_firehose.rs");
        include!("types/iot/topic_rule_http.rs");
        include!("types/iot/topic_rule_http_http_header.rs");
        include!("types/iot/topic_rule_iot_analytic.rs");
        include!("types/iot/topic_rule_iot_event.rs");
        include!("types/iot/topic_rule_kafka.rs");
        include!("types/iot/topic_rule_kafka_header.rs");
        include!("types/iot/topic_rule_kinesis.rs");
        include!("types/iot/topic_rule_lambda.rs");
        include!("types/iot/topic_rule_republish.rs");
        include!("types/iot/topic_rule_s_3.rs");
        include!("types/iot/topic_rule_sns.rs");
        include!("types/iot/topic_rule_sqs.rs");
        include!("types/iot/topic_rule_step_function.rs");
        include!("types/iot/topic_rule_timestream.rs");
        include!("types/iot/topic_rule_timestream_dimension.rs");
        include!("types/iot/topic_rule_timestream_timestamp.rs");
    }
    pub mod ivs {
        include!("types/ivs/recording_configuration_destination_configuration.rs");
        include!("types/ivs/recording_configuration_destination_configuration_s_3.rs");
        include!("types/ivs/recording_configuration_thumbnail_configuration.rs");
    }
    pub mod ivschat {
        include!("types/ivschat/logging_configuration_destination_configuration.rs");
        include!(
            "types/ivschat/logging_configuration_destination_configuration_cloudwatch_logs.rs"
        );
        include!(
            "types/ivschat/logging_configuration_destination_configuration_firehose.rs"
        );
        include!("types/ivschat/logging_configuration_destination_configuration_s_3.rs");
        include!("types/ivschat/room_message_review_handler.rs");
    }
    pub mod kendra {
        include!("types/kendra/data_source_configuration.rs");
        include!("types/kendra/data_source_configuration_s_3_configuration.rs");
        include!(
            "types/kendra/data_source_configuration_s_3_configuration_access_control_list_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_s_3_configuration_documents_metadata_configuration.rs"
        );
        include!("types/kendra/data_source_configuration_web_crawler_configuration.rs");
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_authentication_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_authentication_configuration_basic_authentication.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_proxy_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls_seed_url_configuration.rs"
        );
        include!(
            "types/kendra/data_source_configuration_web_crawler_configuration_urls_site_maps_configuration.rs"
        );
        include!("types/kendra/data_source_custom_document_enrichment_configuration.rs");
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_condition_condition_on_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_target.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_inline_configuration_target_target_document_attribute_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration_invocation_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_post_extraction_hook_configuration_invocation_condition_condition_on_value.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration_invocation_condition.rs"
        );
        include!(
            "types/kendra/data_source_custom_document_enrichment_configuration_pre_extraction_hook_configuration_invocation_condition_condition_on_value.rs"
        );
        include!("types/kendra/experience_configuration.rs");
        include!(
            "types/kendra/experience_configuration_content_source_configuration.rs"
        );
        include!("types/kendra/experience_configuration_user_identity_configuration.rs");
        include!("types/kendra/experience_endpoint.rs");
        include!("types/kendra/faq_s_3_path.rs");
        include!("types/kendra/index_capacity_units.rs");
        include!("types/kendra/index_document_metadata_configuration_update.rs");
        include!(
            "types/kendra/index_document_metadata_configuration_update_relevance.rs"
        );
        include!("types/kendra/index_document_metadata_configuration_update_search.rs");
        include!("types/kendra/index_index_statistic.rs");
        include!("types/kendra/index_index_statistic_faq_statistic.rs");
        include!("types/kendra/index_index_statistic_text_document_statistic.rs");
        include!("types/kendra/index_server_side_encryption_configuration.rs");
        include!("types/kendra/index_user_group_resolution_configuration.rs");
        include!("types/kendra/index_user_token_configurations.rs");
        include!(
            "types/kendra/index_user_token_configurations_json_token_type_configuration.rs"
        );
        include!(
            "types/kendra/index_user_token_configurations_jwt_token_type_configuration.rs"
        );
        include!("types/kendra/query_suggestions_block_list_source_s_3_path.rs");
        include!("types/kendra/thesaurus_source_s_3_path.rs");
        include!("types/kendra/get_experience_configuration.rs");
        include!(
            "types/kendra/get_experience_configuration_content_source_configuration.rs"
        );
        include!(
            "types/kendra/get_experience_configuration_user_identity_configuration.rs"
        );
        include!("types/kendra/get_experience_endpoint.rs");
        include!("types/kendra/get_faq_s_3_path.rs");
        include!("types/kendra/get_index_capacity_unit.rs");
        include!("types/kendra/get_index_document_metadata_configuration_update.rs");
        include!(
            "types/kendra/get_index_document_metadata_configuration_update_relevance.rs"
        );
        include!(
            "types/kendra/get_index_document_metadata_configuration_update_search.rs"
        );
        include!("types/kendra/get_index_index_statistic.rs");
        include!("types/kendra/get_index_index_statistic_faq_statistic.rs");
        include!("types/kendra/get_index_index_statistic_text_document_statistic.rs");
        include!("types/kendra/get_index_server_side_encryption_configuration.rs");
        include!("types/kendra/get_index_user_group_resolution_configuration.rs");
        include!("types/kendra/get_index_user_token_configuration.rs");
        include!(
            "types/kendra/get_index_user_token_configuration_json_token_type_configuration.rs"
        );
        include!(
            "types/kendra/get_index_user_token_configuration_jwt_token_type_configuration.rs"
        );
        include!("types/kendra/get_query_suggestions_block_list_source_s_3_path.rs");
        include!("types/kendra/get_thesaurus_source_s_3_path.rs");
    }
    pub mod keyspaces {
        include!("types/keyspaces/keyspace_replication_specification.rs");
        include!("types/keyspaces/table_capacity_specification.rs");
        include!("types/keyspaces/table_client_side_timestamps.rs");
        include!("types/keyspaces/table_comment.rs");
        include!("types/keyspaces/table_encryption_specification.rs");
        include!("types/keyspaces/table_point_in_time_recovery.rs");
        include!("types/keyspaces/table_schema_definition.rs");
        include!("types/keyspaces/table_schema_definition_clustering_key.rs");
        include!("types/keyspaces/table_schema_definition_column.rs");
        include!("types/keyspaces/table_schema_definition_partition_key.rs");
        include!("types/keyspaces/table_schema_definition_static_column.rs");
        include!("types/keyspaces/table_ttl.rs");
    }
    pub mod kinesis {
        include!("types/kinesis/analytics_application_cloudwatch_logging_options.rs");
        include!("types/kinesis/analytics_application_inputs.rs");
        include!("types/kinesis/analytics_application_inputs_kinesis_firehose.rs");
        include!("types/kinesis/analytics_application_inputs_kinesis_stream.rs");
        include!("types/kinesis/analytics_application_inputs_parallelism.rs");
        include!(
            "types/kinesis/analytics_application_inputs_processing_configuration.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_processing_configuration_lambda.rs"
        );
        include!("types/kinesis/analytics_application_inputs_schema.rs");
        include!("types/kinesis/analytics_application_inputs_schema_record_column.rs");
        include!("types/kinesis/analytics_application_inputs_schema_record_format.rs");
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters_csv.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_schema_record_format_mapping_parameters_json.rs"
        );
        include!(
            "types/kinesis/analytics_application_inputs_starting_position_configuration.rs"
        );
        include!("types/kinesis/analytics_application_output.rs");
        include!("types/kinesis/analytics_application_output_kinesis_firehose.rs");
        include!("types/kinesis/analytics_application_output_kinesis_stream.rs");
        include!("types/kinesis/analytics_application_output_lambda.rs");
        include!("types/kinesis/analytics_application_output_schema.rs");
        include!("types/kinesis/analytics_application_reference_data_sources.rs");
        include!("types/kinesis/analytics_application_reference_data_sources_s_3.rs");
        include!("types/kinesis/analytics_application_reference_data_sources_schema.rs");
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_column.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters_csv.rs"
        );
        include!(
            "types/kinesis/analytics_application_reference_data_sources_schema_record_format_mapping_parameters_json.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_elasticsearch_configuration_vpc_config.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_extended_s_3_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer_hive_json_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_input_format_configuration_deserializer_open_x_json_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer_orc_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_output_format_configuration_serializer_parquet_ser_de.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_data_format_conversion_configuration_schema_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_dynamic_partitioning_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_s_3_backup_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_extended_s_3_configuration_s_3_backup_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_request_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_request_configuration_common_attribute.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_http_endpoint_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_iceberg_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_destination_table_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_iceberg_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_kinesis_source_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_msk_source_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_msk_source_configuration_authentication_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_opensearch_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_document_id_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearch_configuration_vpc_config.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_opensearchserverless_configuration_vpc_config.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_redshift_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_backup_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_backup_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_redshift_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_server_side_encryption.rs");
        include!("types/kinesis/firehose_delivery_stream_snowflake_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_secrets_manager_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_snowflake_role_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_snowflake_configuration_snowflake_vpc_configuration.rs"
        );
        include!("types/kinesis/firehose_delivery_stream_splunk_configuration.rs");
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration_processor.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_processing_configuration_processor_parameter.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_s_3_configuration.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_s_3_configuration_cloudwatch_logging_options.rs"
        );
        include!(
            "types/kinesis/firehose_delivery_stream_splunk_configuration_secrets_manager_configuration.rs"
        );
        include!("types/kinesis/stream_stream_mode_details.rs");
        include!("types/kinesis/get_stream_stream_mode_detail.rs");
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
