pub mod budgets {
    include!("resources/budgets/budget.rs");
    include!("resources/budgets/budget_action.rs");
}
pub mod cfg {
    include!("resources/cfg/aggregate_authorization.rs");
    include!("resources/cfg/configuration_aggregator.rs");
    include!("resources/cfg/conformance_pack.rs");
    include!("resources/cfg/delivery_channel.rs");
    include!("resources/cfg/organization_conformance_pack.rs");
    include!("resources/cfg/organization_custom_policy_rule.rs");
    include!("resources/cfg/organization_custom_rule.rs");
    include!("resources/cfg/organization_managed_rule.rs");
    include!("resources/cfg/recorder.rs");
    include!("resources/cfg/recorder_status.rs");
    include!("resources/cfg/remediation_configuration.rs");
    include!("resources/cfg/retention_configuration.rs");
    include!("resources/cfg/rule.rs");
}
pub mod chatbot {
    include!("resources/chatbot/slack_channel_configuration.rs");
    include!("resources/chatbot/teams_channel_configuration.rs");
}
pub mod chime {
    include!("resources/chime/sdkvoice_global_settings.rs");
    include!("resources/chime/sdkvoice_sip_media_application.rs");
    include!("resources/chime/sdkvoice_sip_rule.rs");
    include!("resources/chime/sdkvoice_voice_profile_domain.rs");
    include!("resources/chime/voice_connector.rs");
    include!("resources/chime/voice_connector_group.rs");
    include!("resources/chime/voice_connector_logging.rs");
    include!("resources/chime/voice_connector_organization.rs");
    include!("resources/chime/voice_connector_streaming.rs");
    include!("resources/chime/voice_connector_termination.rs");
    include!("resources/chime/voice_connector_termination_credentials.rs");
}
pub mod chimesdkmediapipelines {
    include!(
        "resources/chimesdkmediapipelines/media_insights_pipeline_configuration.rs"
    );
}
pub mod cleanrooms {
    include!("resources/cleanrooms/collaboration.rs");
    include!("resources/cleanrooms/configured_table.rs");
}
pub mod cloud9 {
    include!("resources/cloud9/environment_ec_2.rs");
    include!("resources/cloud9/environment_membership.rs");
}
pub mod cloudcontrol {
    include!("resources/cloudcontrol/resource.rs");
}
pub mod cloudformation {
    include!("resources/cloudformation/cloud_formation_type.rs");
    include!("resources/cloudformation/stack.rs");
    include!("resources/cloudformation/stack_instances.rs");
    include!("resources/cloudformation/stack_set.rs");
    include!("resources/cloudformation/stack_set_instance.rs");
}
pub mod cloudfront {
    include!("resources/cloudfront/cache_policy.rs");
    include!("resources/cloudfront/continuous_deployment_policy.rs");
    include!("resources/cloudfront/distribution.rs");
    include!("resources/cloudfront/field_level_encryption_config.rs");
    include!("resources/cloudfront/field_level_encryption_profile.rs");
    include!("resources/cloudfront/function.rs");
    include!("resources/cloudfront/key_group.rs");
    include!("resources/cloudfront/key_value_store.rs");
    include!("resources/cloudfront/keyvaluestore_key.rs");
    include!("resources/cloudfront/monitoring_subscription.rs");
    include!("resources/cloudfront/origin_access_control.rs");
    include!("resources/cloudfront/origin_access_identity.rs");
    include!("resources/cloudfront/origin_request_policy.rs");
    include!("resources/cloudfront/public_key.rs");
    include!("resources/cloudfront/realtime_log_config.rs");
    include!("resources/cloudfront/response_headers_policy.rs");
    include!("resources/cloudfront/vpc_origin.rs");
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
    pub mod budgets {
        include!("functions/budgets/get_budget.rs");
    }
    pub mod chatbot {
        include!("functions/chatbot/get_slack_workspace.rs");
    }
    pub mod cloudcontrol {
        include!("functions/cloudcontrol/get_resource.rs");
    }
    pub mod cloudformation {
        include!("functions/cloudformation/get_cloud_formation_type.rs");
        include!("functions/cloudformation/get_export.rs");
        include!("functions/cloudformation/get_stack.rs");
    }
    pub mod cloudfront {
        include!("functions/cloudfront/get_cache_policy.rs");
        include!("functions/cloudfront/get_distribution.rs");
        include!("functions/cloudfront/get_function.rs");
        include!("functions/cloudfront/get_log_delivery_canonical_user_id.rs");
        include!("functions/cloudfront/get_origin_access_control.rs");
        include!("functions/cloudfront/get_origin_access_identities.rs");
        include!("functions/cloudfront/get_origin_access_identity.rs");
        include!("functions/cloudfront/get_origin_request_policy.rs");
        include!("functions/cloudfront/get_realtime_log_config.rs");
        include!("functions/cloudfront/get_response_headers_policy.rs");
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
    pub mod budgets {
        include!("types/budgets/budget_action_action_threshold.rs");
        include!("types/budgets/budget_action_definition.rs");
        include!("types/budgets/budget_action_definition_iam_action_definition.rs");
        include!("types/budgets/budget_action_definition_scp_action_definition.rs");
        include!("types/budgets/budget_action_definition_ssm_action_definition.rs");
        include!("types/budgets/budget_action_subscriber.rs");
        include!("types/budgets/budget_auto_adjust_data.rs");
        include!("types/budgets/budget_auto_adjust_data_historical_options.rs");
        include!("types/budgets/budget_cost_filter.rs");
        include!("types/budgets/budget_cost_types.rs");
        include!("types/budgets/budget_notification.rs");
        include!("types/budgets/budget_planned_limit.rs");
        include!("types/budgets/get_budget_auto_adjust_data.rs");
        include!("types/budgets/get_budget_auto_adjust_data_historical_option.rs");
        include!("types/budgets/get_budget_budget_limit.rs");
        include!("types/budgets/get_budget_calculated_spend.rs");
        include!("types/budgets/get_budget_calculated_spend_actual_spend.rs");
        include!("types/budgets/get_budget_cost_filter.rs");
        include!("types/budgets/get_budget_cost_type.rs");
        include!("types/budgets/get_budget_notification.rs");
        include!("types/budgets/get_budget_planned_limit.rs");
    }
    pub mod cfg {
        include!("types/cfg/configuration_aggregator_account_aggregation_source.rs");
        include!(
            "types/cfg/configuration_aggregator_organization_aggregation_source.rs"
        );
        include!("types/cfg/conformance_pack_input_parameter.rs");
        include!("types/cfg/delivery_channel_snapshot_delivery_properties.rs");
        include!("types/cfg/organization_conformance_pack_input_parameter.rs");
        include!("types/cfg/recorder_recording_group.rs");
        include!("types/cfg/recorder_recording_group_exclusion_by_resource_type.rs");
        include!("types/cfg/recorder_recording_group_recording_strategy.rs");
        include!("types/cfg/recorder_recording_mode.rs");
        include!("types/cfg/recorder_recording_mode_recording_mode_override.rs");
        include!("types/cfg/remediation_configuration_execution_controls.rs");
        include!(
            "types/cfg/remediation_configuration_execution_controls_ssm_controls.rs"
        );
        include!("types/cfg/remediation_configuration_parameter.rs");
        include!("types/cfg/rule_evaluation_mode.rs");
        include!("types/cfg/rule_scope.rs");
        include!("types/cfg/rule_source.rs");
        include!("types/cfg/rule_source_custom_policy_details.rs");
        include!("types/cfg/rule_source_source_detail.rs");
    }
    pub mod chatbot {
        include!("types/chatbot/slack_channel_configuration_timeouts.rs");
        include!("types/chatbot/teams_channel_configuration_timeouts.rs");
    }
    pub mod chime {
        include!("types/chime/sdkvoice_global_settings_voice_connector.rs");
        include!("types/chime/sdkvoice_sip_media_application_endpoints.rs");
        include!("types/chime/sdkvoice_sip_rule_target_application.rs");
        include!(
            "types/chime/sdkvoice_voice_profile_domain_server_side_encryption_configuration.rs"
        );
        include!("types/chime/voice_connector_group_connector.rs");
        include!("types/chime/voice_connector_organization_route.rs");
        include!(
            "types/chime/voice_connector_streaming_media_insights_configuration.rs"
        );
        include!("types/chime/voice_connector_termination_credentials_credential.rs");
    }
    pub mod chimesdkmediapipelines {
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_call_analytics_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_call_analytics_processor_configuration_post_call_analytics_settings.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_amazon_transcribe_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_kinesis_data_stream_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_lambda_function_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_s_3_recording_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_sns_topic_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_sqs_queue_sink_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_element_voice_analytics_processor_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_issue_detection_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_keyword_match_configuration.rs"
        );
        include!(
            "types/chimesdkmediapipelines/media_insights_pipeline_configuration_real_time_alert_configuration_rule_sentiment_configuration.rs"
        );
    }
    pub mod cleanrooms {
        include!("types/cleanrooms/collaboration_data_encryption_metadata.rs");
        include!("types/cleanrooms/collaboration_member.rs");
        include!("types/cleanrooms/configured_table_table_reference.rs");
    }
    pub mod cloudformation {
        include!("types/cloudformation/cloud_formation_type_logging_config.rs");
        include!("types/cloudformation/stack_instances_deployment_targets.rs");
        include!("types/cloudformation/stack_instances_operation_preferences.rs");
        include!("types/cloudformation/stack_instances_stack_instance_summary.rs");
        include!("types/cloudformation/stack_set_auto_deployment.rs");
        include!("types/cloudformation/stack_set_instance_deployment_targets.rs");
        include!("types/cloudformation/stack_set_instance_operation_preferences.rs");
        include!("types/cloudformation/stack_set_instance_stack_instance_summary.rs");
        include!("types/cloudformation/stack_set_managed_execution.rs");
        include!("types/cloudformation/stack_set_operation_preferences.rs");
        include!("types/cloudformation/get_cloud_formation_type_logging_config.rs");
    }
    pub mod cloudfront {
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config_cookies.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config_headers.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config.rs"
        );
        include!(
            "types/cloudfront/cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config_query_strings.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_staging_distribution_dns_names.rs"
        );
        include!("types/cloudfront/continuous_deployment_policy_traffic_config.rs");
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_header_config.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_weight_config.rs"
        );
        include!(
            "types/cloudfront/continuous_deployment_policy_traffic_config_single_weight_config_session_stickiness_config.rs"
        );
        include!("types/cloudfront/distribution_custom_error_response.rs");
        include!("types/cloudfront/distribution_default_cache_behavior.rs");
        include!(
            "types/cloudfront/distribution_default_cache_behavior_forwarded_values.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_forwarded_values_cookies.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_function_association.rs"
        );
        include!(
            "types/cloudfront/distribution_default_cache_behavior_lambda_function_association.rs"
        );
        include!("types/cloudfront/distribution_logging_config.rs");
        include!("types/cloudfront/distribution_ordered_cache_behavior.rs");
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_forwarded_values.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_forwarded_values_cookies.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_function_association.rs"
        );
        include!(
            "types/cloudfront/distribution_ordered_cache_behavior_lambda_function_association.rs"
        );
        include!("types/cloudfront/distribution_origin.rs");
        include!("types/cloudfront/distribution_origin_custom_header.rs");
        include!("types/cloudfront/distribution_origin_custom_origin_config.rs");
        include!("types/cloudfront/distribution_origin_group.rs");
        include!("types/cloudfront/distribution_origin_group_failover_criteria.rs");
        include!("types/cloudfront/distribution_origin_group_member.rs");
        include!("types/cloudfront/distribution_origin_origin_shield.rs");
        include!("types/cloudfront/distribution_origin_s_3_origin_config.rs");
        include!("types/cloudfront/distribution_origin_vpc_origin_config.rs");
        include!("types/cloudfront/distribution_restrictions.rs");
        include!("types/cloudfront/distribution_restrictions_geo_restriction.rs");
        include!("types/cloudfront/distribution_trusted_key_group.rs");
        include!("types/cloudfront/distribution_trusted_key_group_item.rs");
        include!("types/cloudfront/distribution_trusted_signer.rs");
        include!("types/cloudfront/distribution_trusted_signer_item.rs");
        include!("types/cloudfront/distribution_viewer_certificate.rs");
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config_content_type_profiles.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_content_type_profile_config_content_type_profiles_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config_query_arg_profiles.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_config_query_arg_profile_config_query_arg_profiles_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities_item.rs"
        );
        include!(
            "types/cloudfront/field_level_encryption_profile_encryption_entities_item_field_patterns.rs"
        );
        include!("types/cloudfront/key_value_store_timeouts.rs");
        include!("types/cloudfront/monitoring_subscription_monitoring_subscription.rs");
        include!(
            "types/cloudfront/monitoring_subscription_monitoring_subscription_realtime_metrics_subscription_config.rs"
        );
        include!("types/cloudfront/origin_request_policy_cookies_config.rs");
        include!("types/cloudfront/origin_request_policy_cookies_config_cookies.rs");
        include!("types/cloudfront/origin_request_policy_headers_config.rs");
        include!("types/cloudfront/origin_request_policy_headers_config_headers.rs");
        include!("types/cloudfront/origin_request_policy_query_strings_config.rs");
        include!(
            "types/cloudfront/origin_request_policy_query_strings_config_query_strings.rs"
        );
        include!("types/cloudfront/realtime_log_config_endpoint.rs");
        include!(
            "types/cloudfront/realtime_log_config_endpoint_kinesis_stream_config.rs"
        );
        include!("types/cloudfront/response_headers_policy_cors_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_headers.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_methods.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_allow_origins.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_cors_config_access_control_expose_headers.rs"
        );
        include!("types/cloudfront/response_headers_policy_custom_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_custom_headers_config_item.rs"
        );
        include!("types/cloudfront/response_headers_policy_remove_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_remove_headers_config_item.rs"
        );
        include!("types/cloudfront/response_headers_policy_security_headers_config.rs");
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_content_security_policy.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_content_type_options.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_frame_options.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_referrer_policy.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_strict_transport_security.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_security_headers_config_xss_protection.rs"
        );
        include!(
            "types/cloudfront/response_headers_policy_server_timing_headers_config.rs"
        );
        include!("types/cloudfront/vpc_origin_timeouts.rs");
        include!("types/cloudfront/vpc_origin_vpc_origin_endpoint_config.rs");
        include!(
            "types/cloudfront/vpc_origin_vpc_origin_endpoint_config_origin_ssl_protocols.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_cookies_config_cookie.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_headers_config_header.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config.rs"
        );
        include!(
            "types/cloudfront/get_cache_policy_parameters_in_cache_key_and_forwarded_to_origin_query_strings_config_query_string.rs"
        );
        include!("types/cloudfront/get_origin_request_policy_cookies_config.rs");
        include!("types/cloudfront/get_origin_request_policy_cookies_config_cookie.rs");
        include!("types/cloudfront/get_origin_request_policy_headers_config.rs");
        include!("types/cloudfront/get_origin_request_policy_headers_config_header.rs");
        include!("types/cloudfront/get_origin_request_policy_query_strings_config.rs");
        include!(
            "types/cloudfront/get_origin_request_policy_query_strings_config_query_string.rs"
        );
        include!("types/cloudfront/get_realtime_log_config_endpoint.rs");
        include!(
            "types/cloudfront/get_realtime_log_config_endpoint_kinesis_stream_config.rs"
        );
        include!("types/cloudfront/get_response_headers_policy_cors_config.rs");
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_header.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_method.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_allow_origin.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_cors_config_access_control_expose_header.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_custom_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_custom_headers_config_item.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_remove_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_remove_headers_config_item.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_content_security_policy.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_content_type_option.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_frame_option.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_referrer_policy.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_strict_transport_security.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_security_headers_config_xss_protection.rs"
        );
        include!(
            "types/cloudfront/get_response_headers_policy_server_timing_headers_config.rs"
        );
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
