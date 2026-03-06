pub mod athena {
    include!("resources/athena/data_catalog.rs");
    include!("resources/athena/database.rs");
    include!("resources/athena/named_query.rs");
    include!("resources/athena/prepared_statement.rs");
    include!("resources/athena/workgroup.rs");
}
pub mod auditmanager {
    include!("resources/auditmanager/account_registration.rs");
    include!("resources/auditmanager/assessment.rs");
    include!("resources/auditmanager/assessment_delegation.rs");
    include!("resources/auditmanager/assessment_report.rs");
    include!("resources/auditmanager/control.rs");
    include!("resources/auditmanager/framework.rs");
    include!("resources/auditmanager/framework_share.rs");
    include!("resources/auditmanager/organization_admin_account_registration.rs");
}
pub mod autoscaling {
    include!("resources/autoscaling/attachment.rs");
    include!("resources/autoscaling/group.rs");
    include!("resources/autoscaling/lifecycle_hook.rs");
    include!("resources/autoscaling/notification.rs");
    include!("resources/autoscaling/policy.rs");
    include!("resources/autoscaling/schedule.rs");
    include!("resources/autoscaling/tag.rs");
    include!("resources/autoscaling/traffic_source_attachment.rs");
}
pub mod autoscalingplans {
    include!("resources/autoscalingplans/scaling_plan.rs");
}
pub mod backup {
    include!("resources/backup/framework.rs");
    include!("resources/backup/global_settings.rs");
    include!("resources/backup/logically_air_gapped_vault.rs");
    include!("resources/backup/plan.rs");
    include!("resources/backup/region_settings.rs");
    include!("resources/backup/report_plan.rs");
    include!("resources/backup/restore_testing_plan.rs");
    include!("resources/backup/restore_testing_selection.rs");
    include!("resources/backup/selection.rs");
    include!("resources/backup/vault.rs");
    include!("resources/backup/vault_lock_configuration.rs");
    include!("resources/backup/vault_notifications.rs");
    include!("resources/backup/vault_policy.rs");
}
pub mod batch {
    include!("resources/batch/compute_environment.rs");
    include!("resources/batch/job_definition.rs");
    include!("resources/batch/job_queue.rs");
    include!("resources/batch/scheduling_policy.rs");
}
pub mod bcmdata {
    include!("resources/bcmdata/export.rs");
}
pub mod bedrock {
    include!("resources/bedrock/agent_agent.rs");
    include!("resources/bedrock/agent_agent_action_group.rs");
    include!("resources/bedrock/agent_agent_alias.rs");
    include!("resources/bedrock/agent_agent_knowledge_base_association.rs");
    include!("resources/bedrock/agent_data_source.rs");
    include!("resources/bedrock/agent_knowledge_base.rs");
    include!("resources/bedrock/custom_model.rs");
    include!("resources/bedrock/guardrail.rs");
    include!("resources/bedrock/guardrail_version.rs");
    include!("resources/bedrock/inference_profile.rs");
    include!("resources/bedrock/provisioned_model_throughput.rs");
}
pub mod bedrockmodel {
    include!("resources/bedrockmodel/invocation_logging_configuration.rs");
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
    pub mod athena {
        include!("functions/athena/get_named_query.rs");
    }
    pub mod auditmanager {
        include!("functions/auditmanager/get_control.rs");
        include!("functions/auditmanager/get_framework.rs");
    }
    pub mod autoscaling {
        include!("functions/autoscaling/get_ami_ids.rs");
        include!("functions/autoscaling/get_group.rs");
    }
    pub mod backup {
        include!("functions/backup/get_framework.rs");
        include!("functions/backup/get_plan.rs");
        include!("functions/backup/get_report_plan.rs");
        include!("functions/backup/get_selection.rs");
        include!("functions/backup/get_vault.rs");
    }
    pub mod batch {
        include!("functions/batch/get_compute_environment.rs");
        include!("functions/batch/get_job_definition.rs");
        include!("functions/batch/get_job_queue.rs");
        include!("functions/batch/get_scheduling_policy.rs");
    }
    pub mod bedrock {
        include!("functions/bedrock/get_agent_agent_versions.rs");
        include!("functions/bedrock/get_custom_model.rs");
        include!("functions/bedrock/get_custom_models.rs");
        include!("functions/bedrock/get_inference_profile.rs");
        include!("functions/bedrock/get_inference_profiles.rs");
    }
    pub mod bedrockfoundation {
        include!("functions/bedrockfoundation/get_model.rs");
        include!("functions/bedrockfoundation/get_models.rs");
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
    pub mod athena {
        include!("types/athena/database_acl_configuration.rs");
        include!("types/athena/database_encryption_configuration.rs");
        include!("types/athena/workgroup_configuration.rs");
        include!("types/athena/workgroup_configuration_engine_version.rs");
        include!("types/athena/workgroup_configuration_result_configuration.rs");
        include!(
            "types/athena/workgroup_configuration_result_configuration_acl_configuration.rs"
        );
        include!(
            "types/athena/workgroup_configuration_result_configuration_encryption_configuration.rs"
        );
    }
    pub mod auditmanager {
        include!("types/auditmanager/assessment_assessment_reports_destination.rs");
        include!("types/auditmanager/assessment_role.rs");
        include!("types/auditmanager/assessment_roles_all.rs");
        include!("types/auditmanager/assessment_scope.rs");
        include!("types/auditmanager/assessment_scope_aws_account.rs");
        include!("types/auditmanager/assessment_scope_aws_service.rs");
        include!("types/auditmanager/control_control_mapping_source.rs");
        include!("types/auditmanager/control_control_mapping_source_source_keyword.rs");
        include!("types/auditmanager/framework_control_set.rs");
        include!("types/auditmanager/framework_control_set_control.rs");
        include!("types/auditmanager/get_control_control_mapping_source.rs");
        include!(
            "types/auditmanager/get_control_control_mapping_source_source_keyword.rs"
        );
        include!("types/auditmanager/get_framework_control_set.rs");
        include!("types/auditmanager/get_framework_control_set_control.rs");
    }
    pub mod autoscaling {
        include!("types/autoscaling/group_availability_zone_distribution.rs");
        include!("types/autoscaling/group_initial_lifecycle_hook.rs");
        include!("types/autoscaling/group_instance_maintenance_policy.rs");
        include!("types/autoscaling/group_instance_refresh.rs");
        include!("types/autoscaling/group_instance_refresh_preferences.rs");
        include!(
            "types/autoscaling/group_instance_refresh_preferences_alarm_specification.rs"
        );
        include!("types/autoscaling/group_launch_template.rs");
        include!("types/autoscaling/group_mixed_instances_policy.rs");
        include!(
            "types/autoscaling/group_mixed_instances_policy_instances_distribution.rs"
        );
        include!("types/autoscaling/group_mixed_instances_policy_launch_template.rs");
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_launch_template_specification.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_accelerator_count.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_accelerator_total_memory_mib.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_baseline_ebs_bandwidth_mbps.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_memory_gib_per_vcpu.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_memory_mib.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_network_bandwidth_gbps.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_network_interface_count.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_total_local_storage_gb.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_instance_requirements_vcpu_count.rs"
        );
        include!(
            "types/autoscaling/group_mixed_instances_policy_launch_template_override_launch_template_specification.rs"
        );
        include!("types/autoscaling/group_tag.rs");
        include!("types/autoscaling/group_traffic_source.rs");
        include!("types/autoscaling/group_warm_pool.rs");
        include!("types/autoscaling/group_warm_pool_instance_reuse_policy.rs");
        include!("types/autoscaling/policy_predictive_scaling_configuration.rs");
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_capacity_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_capacity_metric_specification_metric_data_query.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_capacity_metric_specification_metric_data_query_metric_stat.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_capacity_metric_specification_metric_data_query_metric_stat_metric.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_capacity_metric_specification_metric_data_query_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_load_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_load_metric_specification_metric_data_query.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_load_metric_specification_metric_data_query_metric_stat.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_load_metric_specification_metric_data_query_metric_stat_metric.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_load_metric_specification_metric_data_query_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_scaling_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_scaling_metric_specification_metric_data_query.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_scaling_metric_specification_metric_data_query_metric_stat.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_scaling_metric_specification_metric_data_query_metric_stat_metric.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_customized_scaling_metric_specification_metric_data_query_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_predefined_load_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_predefined_metric_pair_specification.rs"
        );
        include!(
            "types/autoscaling/policy_predictive_scaling_configuration_metric_specification_predefined_scaling_metric_specification.rs"
        );
        include!("types/autoscaling/policy_step_adjustment.rs");
        include!("types/autoscaling/policy_target_tracking_configuration.rs");
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification_metric.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification_metric_dimension.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification_metric_metric_stat.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification_metric_metric_stat_metric.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_customized_metric_specification_metric_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/autoscaling/policy_target_tracking_configuration_predefined_metric_specification.rs"
        );
        include!("types/autoscaling/tag_tag.rs");
        include!("types/autoscaling/traffic_source_attachment_traffic_source.rs");
        include!("types/autoscaling/get_ami_ids_filter.rs");
        include!("types/autoscaling/get_group_instance_maintenance_policy.rs");
        include!("types/autoscaling/get_group_launch_template.rs");
        include!("types/autoscaling/get_group_mixed_instances_policy.rs");
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_instances_distribution.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_launch_template_specification.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_accelerator_count.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_accelerator_total_memory_mib.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_baseline_ebs_bandwidth_mbp.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_memory_gib_per_vcpus.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_memory_mib.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_network_bandwidth_gbp.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_network_interface_count.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_total_local_storage_gb.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_instance_requirement_vcpu_count.rs"
        );
        include!(
            "types/autoscaling/get_group_mixed_instances_policy_launch_template_override_launch_template_specification.rs"
        );
        include!("types/autoscaling/get_group_tag.rs");
        include!("types/autoscaling/get_group_traffic_source.rs");
        include!("types/autoscaling/get_group_warm_pool.rs");
        include!("types/autoscaling/get_group_warm_pool_instance_reuse_policy.rs");
    }
    pub mod autoscalingplans {
        include!("types/autoscalingplans/scaling_plan_application_source.rs");
        include!("types/autoscalingplans/scaling_plan_application_source_tag_filter.rs");
        include!("types/autoscalingplans/scaling_plan_scaling_instruction.rs");
        include!(
            "types/autoscalingplans/scaling_plan_scaling_instruction_customized_load_metric_specification.rs"
        );
        include!(
            "types/autoscalingplans/scaling_plan_scaling_instruction_predefined_load_metric_specification.rs"
        );
        include!(
            "types/autoscalingplans/scaling_plan_scaling_instruction_target_tracking_configuration.rs"
        );
        include!(
            "types/autoscalingplans/scaling_plan_scaling_instruction_target_tracking_configuration_customized_scaling_metric_specification.rs"
        );
        include!(
            "types/autoscalingplans/scaling_plan_scaling_instruction_target_tracking_configuration_predefined_scaling_metric_specification.rs"
        );
    }
    pub mod backup {
        include!("types/backup/framework_control.rs");
        include!("types/backup/framework_control_input_parameter.rs");
        include!("types/backup/framework_control_scope.rs");
        include!("types/backup/logically_air_gapped_vault_timeouts.rs");
        include!("types/backup/plan_advanced_backup_setting.rs");
        include!("types/backup/plan_rule.rs");
        include!("types/backup/plan_rule_copy_action.rs");
        include!("types/backup/plan_rule_copy_action_lifecycle.rs");
        include!("types/backup/plan_rule_lifecycle.rs");
        include!("types/backup/report_plan_report_delivery_channel.rs");
        include!("types/backup/report_plan_report_setting.rs");
        include!("types/backup/restore_testing_plan_recovery_point_selection.rs");
        include!(
            "types/backup/restore_testing_selection_protected_resource_conditions.rs"
        );
        include!(
            "types/backup/restore_testing_selection_protected_resource_conditions_string_equal.rs"
        );
        include!(
            "types/backup/restore_testing_selection_protected_resource_conditions_string_not_equal.rs"
        );
        include!("types/backup/selection_condition.rs");
        include!("types/backup/selection_condition_string_equal.rs");
        include!("types/backup/selection_condition_string_like.rs");
        include!("types/backup/selection_condition_string_not_equal.rs");
        include!("types/backup/selection_condition_string_not_like.rs");
        include!("types/backup/selection_selection_tag.rs");
        include!("types/backup/get_framework_control.rs");
        include!("types/backup/get_framework_control_input_parameter.rs");
        include!("types/backup/get_framework_control_scope.rs");
        include!("types/backup/get_plan_rule.rs");
        include!("types/backup/get_plan_rule_copy_action.rs");
        include!("types/backup/get_plan_rule_copy_action_lifecycle.rs");
        include!("types/backup/get_plan_rule_lifecycle.rs");
        include!("types/backup/get_report_plan_report_delivery_channel.rs");
        include!("types/backup/get_report_plan_report_setting.rs");
    }
    pub mod batch {
        include!("types/batch/compute_environment_compute_resources.rs");
        include!(
            "types/batch/compute_environment_compute_resources_ec_2_configuration.rs"
        );
        include!("types/batch/compute_environment_compute_resources_launch_template.rs");
        include!("types/batch/compute_environment_eks_configuration.rs");
        include!("types/batch/compute_environment_update_policy.rs");
        include!("types/batch/job_definition_eks_properties.rs");
        include!("types/batch/job_definition_eks_properties_pod_properties.rs");
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_containers.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_containers_env.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_containers_resources.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_containers_security_context.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_containers_volume_mount.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_image_pull_secret.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_init_container.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_init_container_env.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_init_container_resources.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_init_container_security_context.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_init_container_volume_mount.rs"
        );
        include!("types/batch/job_definition_eks_properties_pod_properties_metadata.rs");
        include!("types/batch/job_definition_eks_properties_pod_properties_volume.rs");
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_volume_empty_dir.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_volume_host_path.rs"
        );
        include!(
            "types/batch/job_definition_eks_properties_pod_properties_volume_secret.rs"
        );
        include!("types/batch/job_definition_retry_strategy.rs");
        include!("types/batch/job_definition_retry_strategy_evaluate_on_exit.rs");
        include!("types/batch/job_definition_timeout.rs");
        include!("types/batch/job_queue_compute_environment_order.rs");
        include!("types/batch/job_queue_job_state_time_limit_action.rs");
        include!("types/batch/job_queue_timeouts.rs");
        include!("types/batch/scheduling_policy_fair_share_policy.rs");
        include!(
            "types/batch/scheduling_policy_fair_share_policy_share_distribution.rs"
        );
        include!("types/batch/get_compute_environment_update_policy.rs");
        include!("types/batch/get_job_definition_eks_property.rs");
        include!("types/batch/get_job_definition_eks_property_pod_property.rs");
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_container.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_container_env.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_container_resource.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_container_security_context.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_container_volume_mount.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_image_pull_secret.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_init_container.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_init_container_env.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_init_container_resource.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_init_container_security_context.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_init_container_volume_mount.rs"
        );
        include!("types/batch/get_job_definition_eks_property_pod_property_metadata.rs");
        include!("types/batch/get_job_definition_eks_property_pod_property_volume.rs");
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_volume_empty_dir.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_volume_host_path.rs"
        );
        include!(
            "types/batch/get_job_definition_eks_property_pod_property_volume_secret.rs"
        );
        include!("types/batch/get_job_definition_node_property.rs");
        include!("types/batch/get_job_definition_node_property_node_range_property.rs");
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_environment.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_ephemeral_storage.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_fargate_platform_configuration.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_linux_parameter.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_linux_parameter_device.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_linux_parameter_tmpf.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_log_configuration.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_log_configuration_secret_option.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_mount_point.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_network_configuration.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_resource_requirement.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_runtime_platform.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_secret.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_ulimit.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_volume.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_volume_efs_volume_configuration.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_volume_efs_volume_configuration_authorization_config.rs"
        );
        include!(
            "types/batch/get_job_definition_node_property_node_range_property_container_volume_host.rs"
        );
        include!("types/batch/get_job_definition_retry_strategy.rs");
        include!("types/batch/get_job_definition_retry_strategy_evaluate_on_exit.rs");
        include!("types/batch/get_job_definition_timeout.rs");
        include!("types/batch/get_job_queue_compute_environment_order.rs");
        include!("types/batch/get_job_queue_job_state_time_limit_action.rs");
        include!("types/batch/get_scheduling_policy_fair_share_policy.rs");
        include!(
            "types/batch/get_scheduling_policy_fair_share_policy_share_distribution.rs"
        );
    }
    pub mod bcmdata {
        include!("types/bcmdata/export_export.rs");
        include!("types/bcmdata/export_export_data_query.rs");
        include!("types/bcmdata/export_export_destination_configuration.rs");
        include!(
            "types/bcmdata/export_export_destination_configuration_s_3_destination.rs"
        );
        include!(
            "types/bcmdata/export_export_destination_configuration_s_3_destination_s_3_output_configuration.rs"
        );
        include!("types/bcmdata/export_export_refresh_cadence.rs");
        include!("types/bcmdata/export_timeouts.rs");
    }
    pub mod bedrock {
        include!("types/bedrock/agent_agent_action_group_action_group_executor.rs");
        include!("types/bedrock/agent_agent_action_group_api_schema.rs");
        include!("types/bedrock/agent_agent_action_group_api_schema_s_3.rs");
        include!("types/bedrock/agent_agent_action_group_function_schema.rs");
        include!(
            "types/bedrock/agent_agent_action_group_function_schema_member_functions.rs"
        );
        include!(
            "types/bedrock/agent_agent_action_group_function_schema_member_functions_function.rs"
        );
        include!(
            "types/bedrock/agent_agent_action_group_function_schema_member_functions_function_parameter.rs"
        );
        include!("types/bedrock/agent_agent_action_group_timeouts.rs");
        include!("types/bedrock/agent_agent_alias_routing_configuration.rs");
        include!("types/bedrock/agent_agent_alias_timeouts.rs");
        include!("types/bedrock/agent_agent_guardrail_configuration.rs");
        include!("types/bedrock/agent_agent_knowledge_base_association_timeouts.rs");
        include!("types/bedrock/agent_agent_prompt_override_configuration.rs");
        include!(
            "types/bedrock/agent_agent_prompt_override_configuration_prompt_configuration.rs"
        );
        include!(
            "types/bedrock/agent_agent_prompt_override_configuration_prompt_configuration_inference_configuration.rs"
        );
        include!("types/bedrock/agent_agent_timeouts.rs");
        include!("types/bedrock/agent_data_source_data_source_configuration.rs");
        include!(
            "types/bedrock/agent_data_source_data_source_configuration_s_3_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_server_side_encryption_configuration.rs"
        );
        include!("types/bedrock/agent_data_source_timeouts.rs");
        include!("types/bedrock/agent_data_source_vector_ingestion_configuration.rs");
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_chunking_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_chunking_configuration_fixed_size_chunking_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_chunking_configuration_hierarchical_chunking_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_chunking_configuration_hierarchical_chunking_configuration_level_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_chunking_configuration_semantic_chunking_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration_intermediate_storage.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration_intermediate_storage_s_3_location.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration_transformation.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration_transformation_transformation_function.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_custom_transformation_configuration_transformation_transformation_function_transformation_lambda_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_parsing_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_parsing_configuration_bedrock_foundation_model_configuration.rs"
        );
        include!(
            "types/bedrock/agent_data_source_vector_ingestion_configuration_parsing_configuration_bedrock_foundation_model_configuration_parsing_prompt.rs"
        );
        include!("types/bedrock/agent_knowledge_base_knowledge_base_configuration.rs");
        include!(
            "types/bedrock/agent_knowledge_base_knowledge_base_configuration_vector_knowledge_base_configuration.rs"
        );
        include!("types/bedrock/agent_knowledge_base_storage_configuration.rs");
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_opensearch_serverless_configuration.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_opensearch_serverless_configuration_field_mapping.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_pinecone_configuration.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_pinecone_configuration_field_mapping.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_rds_configuration.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_rds_configuration_field_mapping.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_redis_enterprise_cloud_configuration.rs"
        );
        include!(
            "types/bedrock/agent_knowledge_base_storage_configuration_redis_enterprise_cloud_configuration_field_mapping.rs"
        );
        include!("types/bedrock/agent_knowledge_base_timeouts.rs");
        include!("types/bedrock/custom_model_output_data_config.rs");
        include!("types/bedrock/custom_model_timeouts.rs");
        include!("types/bedrock/custom_model_training_data_config.rs");
        include!("types/bedrock/custom_model_training_metric.rs");
        include!("types/bedrock/custom_model_validation_data_config.rs");
        include!("types/bedrock/custom_model_validation_data_config_validator.rs");
        include!("types/bedrock/custom_model_validation_metric.rs");
        include!("types/bedrock/custom_model_vpc_config.rs");
        include!("types/bedrock/guardrail_content_policy_config.rs");
        include!("types/bedrock/guardrail_content_policy_config_filters_config.rs");
        include!("types/bedrock/guardrail_contextual_grounding_policy_config.rs");
        include!(
            "types/bedrock/guardrail_contextual_grounding_policy_config_filters_config.rs"
        );
        include!("types/bedrock/guardrail_sensitive_information_policy_config.rs");
        include!(
            "types/bedrock/guardrail_sensitive_information_policy_config_pii_entities_config.rs"
        );
        include!(
            "types/bedrock/guardrail_sensitive_information_policy_config_regexes_config.rs"
        );
        include!("types/bedrock/guardrail_timeouts.rs");
        include!("types/bedrock/guardrail_topic_policy_config.rs");
        include!("types/bedrock/guardrail_topic_policy_config_topics_config.rs");
        include!("types/bedrock/guardrail_version_timeouts.rs");
        include!("types/bedrock/guardrail_word_policy_config.rs");
        include!(
            "types/bedrock/guardrail_word_policy_config_managed_word_lists_config.rs"
        );
        include!("types/bedrock/guardrail_word_policy_config_words_config.rs");
        include!("types/bedrock/inference_profile_model.rs");
        include!("types/bedrock/inference_profile_model_source.rs");
        include!("types/bedrock/inference_profile_timeouts.rs");
        include!("types/bedrock/provisioned_model_throughput_timeouts.rs");
        include!("types/bedrock/get_agent_agent_versions_agent_version_summary.rs");
        include!(
            "types/bedrock/get_agent_agent_versions_agent_version_summary_guardrail_configuration.rs"
        );
        include!("types/bedrock/get_custom_model_output_data_config.rs");
        include!("types/bedrock/get_custom_model_training_data_config.rs");
        include!("types/bedrock/get_custom_model_training_metric.rs");
        include!("types/bedrock/get_custom_model_validation_data_config.rs");
        include!("types/bedrock/get_custom_model_validation_data_config_validator.rs");
        include!("types/bedrock/get_custom_model_validation_metric.rs");
        include!("types/bedrock/get_custom_models_model_summary.rs");
        include!("types/bedrock/get_inference_profile_model.rs");
        include!("types/bedrock/get_inference_profiles_inference_profile_summary.rs");
        include!(
            "types/bedrock/get_inference_profiles_inference_profile_summary_model.rs"
        );
    }
    pub mod bedrockfoundation {
        include!("types/bedrockfoundation/get_models_model_summary.rs");
    }
    pub mod bedrockmodel {
        include!(
            "types/bedrockmodel/invocation_logging_configuration_logging_config.rs"
        );
        include!(
            "types/bedrockmodel/invocation_logging_configuration_logging_config_cloudwatch_config.rs"
        );
        include!(
            "types/bedrockmodel/invocation_logging_configuration_logging_config_cloudwatch_config_large_data_delivery_s_3_config.rs"
        );
        include!(
            "types/bedrockmodel/invocation_logging_configuration_logging_config_s_3_config.rs"
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
