pub mod rds {
    include!("resources/rds/certificate.rs");
    include!("resources/rds/cluster.rs");
    include!("resources/rds/cluster_activity_stream.rs");
    include!("resources/rds/cluster_endpoint.rs");
    include!("resources/rds/cluster_instance.rs");
    include!("resources/rds/cluster_parameter_group.rs");
    include!("resources/rds/cluster_role_association.rs");
    include!("resources/rds/cluster_snapshot.rs");
    include!("resources/rds/cluster_snapshot_copy.rs");
    include!("resources/rds/custom_db_engine_version.rs");
    include!("resources/rds/event_subscription.rs");
    include!("resources/rds/export_task.rs");
    include!("resources/rds/global_cluster.rs");
    include!("resources/rds/instance.rs");
    include!("resources/rds/instance_automated_backups_replication.rs");
    include!("resources/rds/instance_desired_state.rs");
    include!("resources/rds/integration.rs");
    include!("resources/rds/option_group.rs");
    include!("resources/rds/parameter_group.rs");
    include!("resources/rds/proxy.rs");
    include!("resources/rds/proxy_default_target_group.rs");
    include!("resources/rds/proxy_endpoint.rs");
    include!("resources/rds/proxy_target.rs");
    include!("resources/rds/reserved_instance.rs");
    include!("resources/rds/role_association.rs");
    include!("resources/rds/snapshot.rs");
    include!("resources/rds/snapshot_copy.rs");
    include!("resources/rds/subnet_group.rs");
}
pub mod redshift {
    include!("resources/redshift/authentication_profile.rs");
    include!("resources/redshift/cluster.rs");
    include!("resources/redshift/cluster_iam_roles.rs");
    include!("resources/redshift/cluster_snapshot.rs");
    include!("resources/redshift/data_share_authorization.rs");
    include!("resources/redshift/data_share_consumer_association.rs");
    include!("resources/redshift/endpoint_access.rs");
    include!("resources/redshift/endpoint_authorization.rs");
    include!("resources/redshift/event_subscription.rs");
    include!("resources/redshift/hsm_client_certificate.rs");
    include!("resources/redshift/hsm_configuration.rs");
    include!("resources/redshift/logging.rs");
    include!("resources/redshift/parameter_group.rs");
    include!("resources/redshift/partner.rs");
    include!("resources/redshift/resource_policy.rs");
    include!("resources/redshift/scheduled_action.rs");
    include!("resources/redshift/snapshot_copy.rs");
    include!("resources/redshift/snapshot_copy_grant.rs");
    include!("resources/redshift/snapshot_schedule.rs");
    include!("resources/redshift/snapshot_schedule_association.rs");
    include!("resources/redshift/subnet_group.rs");
    include!("resources/redshift/usage_limit.rs");
}
pub mod redshiftdata {
    include!("resources/redshiftdata/statement.rs");
}
pub mod redshiftserverless {
    include!("resources/redshiftserverless/custom_domain_association.rs");
    include!("resources/redshiftserverless/endpoint_access.rs");
    include!("resources/redshiftserverless/namespace.rs");
    include!("resources/redshiftserverless/resource_policy.rs");
    include!("resources/redshiftserverless/snapshot.rs");
    include!("resources/redshiftserverless/usage_limit.rs");
    include!("resources/redshiftserverless/workgroup.rs");
}
pub mod rekognition {
    include!("resources/rekognition/collection.rs");
    include!("resources/rekognition/project.rs");
    include!("resources/rekognition/stream_processor.rs");
}
pub mod resiliencehub {
    include!("resources/resiliencehub/resiliency_policy.rs");
}
pub mod resourceexplorer {
    include!("resources/resourceexplorer/index.rs");
    include!("resources/resourceexplorer/view.rs");
}
pub mod resourcegroups {
    include!("resources/resourcegroups/group.rs");
    include!("resources/resourcegroups/resource.rs");
}
pub mod rolesanywhere {
    include!("resources/rolesanywhere/profile.rs");
    include!("resources/rolesanywhere/trust_anchor.rs");
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
    pub mod rds {
        include!("functions/rds/get_certificate.rs");
        include!("functions/rds/get_cluster.rs");
        include!("functions/rds/get_cluster_parameter_group.rs");
        include!("functions/rds/get_cluster_snapshot.rs");
        include!("functions/rds/get_clusters.rs");
        include!("functions/rds/get_engine_version.rs");
        include!("functions/rds/get_event_categories.rs");
        include!("functions/rds/get_instance.rs");
        include!("functions/rds/get_instances.rs");
        include!("functions/rds/get_orderable_db_instance.rs");
        include!("functions/rds/get_parameter_group.rs");
        include!("functions/rds/get_proxy.rs");
        include!("functions/rds/get_reserved_instance_offering.rs");
        include!("functions/rds/get_snapshot.rs");
        include!("functions/rds/get_subnet_group.rs");
    }
    pub mod redshift {
        include!("functions/redshift/get_cluster.rs");
        include!("functions/redshift/get_cluster_credentials.rs");
        include!("functions/redshift/get_data_shares.rs");
        include!("functions/redshift/get_orderable_cluster.rs");
        include!("functions/redshift/get_producer_data_shares.rs");
        include!("functions/redshift/get_service_account.rs");
        include!("functions/redshift/get_subnet_group.rs");
    }
    pub mod redshiftserverless {
        include!("functions/redshiftserverless/get_credentials.rs");
        include!("functions/redshiftserverless/get_namespace.rs");
        include!("functions/redshiftserverless/get_workgroup.rs");
    }
    pub mod resourceexplorer {
        include!("functions/resourceexplorer/search.rs");
    }
    pub mod resourcegroupstaggingapi {
        include!("functions/resourcegroupstaggingapi/get_resources.rs");
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
    pub mod rds {
        include!("types/rds/cluster_master_user_secret.rs");
        include!("types/rds/cluster_parameter_group_parameter.rs");
        include!("types/rds/cluster_restore_to_point_in_time.rs");
        include!("types/rds/cluster_s_3_import.rs");
        include!("types/rds/cluster_scaling_configuration.rs");
        include!("types/rds/cluster_serverlessv_2_scaling_configuration.rs");
        include!("types/rds/cluster_snapshot_copy_timeouts.rs");
        include!("types/rds/export_task_timeouts.rs");
        include!("types/rds/global_cluster_global_cluster_member.rs");
        include!("types/rds/instance_blue_green_update.rs");
        include!("types/rds/instance_desired_state_timeouts.rs");
        include!("types/rds/instance_listener_endpoint.rs");
        include!("types/rds/instance_master_user_secret.rs");
        include!("types/rds/instance_restore_to_point_in_time.rs");
        include!("types/rds/instance_s_3_import.rs");
        include!("types/rds/integration_timeouts.rs");
        include!("types/rds/option_group_option.rs");
        include!("types/rds/option_group_option_option_setting.rs");
        include!("types/rds/parameter_group_parameter.rs");
        include!("types/rds/proxy_auth.rs");
        include!("types/rds/proxy_default_target_group_connection_pool_config.rs");
        include!("types/rds/reserved_instance_recurring_charge.rs");
        include!("types/rds/get_cluster_master_user_secret.rs");
        include!("types/rds/get_clusters_filter.rs");
        include!("types/rds/get_engine_version_filter.rs");
        include!("types/rds/get_instance_master_user_secret.rs");
        include!("types/rds/get_instances_filter.rs");
        include!("types/rds/get_proxy_auth.rs");
    }
    pub mod redshift {
        include!("types/redshift/cluster_cluster_node.rs");
        include!("types/redshift/cluster_logging.rs");
        include!("types/redshift/cluster_snapshot_copy.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint.rs");
        include!("types/redshift/endpoint_access_vpc_endpoint_network_interface.rs");
        include!("types/redshift/parameter_group_parameter.rs");
        include!("types/redshift/scheduled_action_target_action.rs");
        include!("types/redshift/scheduled_action_target_action_pause_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resize_cluster.rs");
        include!("types/redshift/scheduled_action_target_action_resume_cluster.rs");
        include!("types/redshift/get_cluster_cluster_node.rs");
        include!("types/redshift/get_data_shares_data_share.rs");
        include!("types/redshift/get_producer_data_shares_data_share.rs");
    }
    pub mod redshiftdata {
        include!("types/redshiftdata/statement_parameter.rs");
    }
    pub mod redshiftserverless {
        include!("types/redshiftserverless/endpoint_access_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/endpoint_access_vpc_endpoint_network_interface.rs"
        );
        include!("types/redshiftserverless/workgroup_config_parameter.rs");
        include!("types/redshiftserverless/workgroup_endpoint.rs");
        include!("types/redshiftserverless/workgroup_endpoint_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/workgroup_endpoint_vpc_endpoint_network_interface.rs"
        );
        include!("types/redshiftserverless/get_workgroup_endpoint.rs");
        include!("types/redshiftserverless/get_workgroup_endpoint_vpc_endpoint.rs");
        include!(
            "types/redshiftserverless/get_workgroup_endpoint_vpc_endpoint_network_interface.rs"
        );
    }
    pub mod rekognition {
        include!("types/rekognition/collection_timeouts.rs");
        include!("types/rekognition/project_timeouts.rs");
        include!("types/rekognition/stream_processor_data_sharing_preference.rs");
        include!("types/rekognition/stream_processor_input.rs");
        include!("types/rekognition/stream_processor_input_kinesis_video_stream.rs");
        include!("types/rekognition/stream_processor_notification_channel.rs");
        include!("types/rekognition/stream_processor_output.rs");
        include!("types/rekognition/stream_processor_output_kinesis_data_stream.rs");
        include!("types/rekognition/stream_processor_output_s_3_destination.rs");
        include!("types/rekognition/stream_processor_regions_of_interest.rs");
        include!(
            "types/rekognition/stream_processor_regions_of_interest_bounding_box.rs"
        );
        include!("types/rekognition/stream_processor_regions_of_interest_polygon.rs");
        include!("types/rekognition/stream_processor_settings.rs");
        include!("types/rekognition/stream_processor_settings_connected_home.rs");
        include!("types/rekognition/stream_processor_settings_face_search.rs");
        include!("types/rekognition/stream_processor_timeouts.rs");
    }
    pub mod resiliencehub {
        include!("types/resiliencehub/resiliency_policy_policy.rs");
        include!("types/resiliencehub/resiliency_policy_policy_az.rs");
        include!("types/resiliencehub/resiliency_policy_policy_hardware.rs");
        include!("types/resiliencehub/resiliency_policy_policy_region.rs");
        include!("types/resiliencehub/resiliency_policy_policy_software.rs");
        include!("types/resiliencehub/resiliency_policy_timeouts.rs");
    }
    pub mod resourceexplorer {
        include!("types/resourceexplorer/index_timeouts.rs");
        include!("types/resourceexplorer/search_resource.rs");
        include!("types/resourceexplorer/search_resource_count.rs");
        include!("types/resourceexplorer/search_resource_property.rs");
        include!("types/resourceexplorer/view_filters.rs");
        include!("types/resourceexplorer/view_included_property.rs");
    }
    pub mod resourcegroups {
        include!("types/resourcegroups/group_configuration.rs");
        include!("types/resourcegroups/group_configuration_parameter.rs");
        include!("types/resourcegroups/group_resource_query.rs");
    }
    pub mod resourcegroupstaggingapi {
        include!(
            "types/resourcegroupstaggingapi/get_resources_resource_tag_mapping_list.rs"
        );
        include!(
            "types/resourcegroupstaggingapi/get_resources_resource_tag_mapping_list_compliance_detail.rs"
        );
        include!("types/resourcegroupstaggingapi/get_resources_tag_filter.rs");
    }
    pub mod rolesanywhere {
        include!("types/rolesanywhere/trust_anchor_notification_setting.rs");
        include!("types/rolesanywhere/trust_anchor_source.rs");
        include!("types/rolesanywhere/trust_anchor_source_source_data.rs");
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
