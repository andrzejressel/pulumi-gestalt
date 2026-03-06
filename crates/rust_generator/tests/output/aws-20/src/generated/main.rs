pub mod ssoadmin {
    include!("resources/ssoadmin/account_assignment.rs");
    include!("resources/ssoadmin/application.rs");
    include!("resources/ssoadmin/application_access_scope.rs");
    include!("resources/ssoadmin/application_assignment.rs");
    include!("resources/ssoadmin/application_assignment_configuration.rs");
    include!("resources/ssoadmin/customer_managed_policy_attachment.rs");
    include!("resources/ssoadmin/instance_access_control_attributes.rs");
    include!("resources/ssoadmin/managed_policy_attachment.rs");
    include!("resources/ssoadmin/permission_set.rs");
    include!("resources/ssoadmin/permission_set_inline_policy.rs");
    include!("resources/ssoadmin/permissions_boundary_attachment.rs");
    include!("resources/ssoadmin/trusted_token_issuer.rs");
}
pub mod storagegateway {
    include!("resources/storagegateway/cache.rs");
    include!("resources/storagegateway/caches_iscsi_volume.rs");
    include!("resources/storagegateway/file_system_association.rs");
    include!("resources/storagegateway/gateway.rs");
    include!("resources/storagegateway/nfs_file_share.rs");
    include!("resources/storagegateway/smb_file_share.rs");
    include!("resources/storagegateway/stored_iscsi_volume.rs");
    include!("resources/storagegateway/tape_pool.rs");
    include!("resources/storagegateway/upload_buffer.rs");
    include!("resources/storagegateway/working_storage.rs");
}
pub mod swf {
    include!("resources/swf/domain.rs");
}
pub mod synthetics {
    include!("resources/synthetics/canary.rs");
    include!("resources/synthetics/group.rs");
    include!("resources/synthetics/group_association.rs");
}
pub mod timestreaminfluxdb {
    include!("resources/timestreaminfluxdb/db_instance.rs");
}
pub mod timestreamwrite {
    include!("resources/timestreamwrite/database.rs");
    include!("resources/timestreamwrite/table.rs");
}
pub mod transcribe {
    include!("resources/transcribe/language_model.rs");
    include!("resources/transcribe/medical_vocabulary.rs");
    include!("resources/transcribe/vocabulary.rs");
    include!("resources/transcribe/vocabulary_filter.rs");
}
pub mod transfer {
    include!("resources/transfer/access.rs");
    include!("resources/transfer/agreement.rs");
    include!("resources/transfer/certificate.rs");
    include!("resources/transfer/connector.rs");
    include!("resources/transfer/profile.rs");
    include!("resources/transfer/server.rs");
    include!("resources/transfer/ssh_key.rs");
    include!("resources/transfer/tag.rs");
    include!("resources/transfer/user.rs");
    include!("resources/transfer/workflow.rs");
}
pub mod verifiedaccess {
    include!("resources/verifiedaccess/endpoint.rs");
    include!("resources/verifiedaccess/group.rs");
    include!("resources/verifiedaccess/instance.rs");
    include!("resources/verifiedaccess/instance_logging_configuration.rs");
    include!("resources/verifiedaccess/instance_trust_provider_attachment.rs");
    include!("resources/verifiedaccess/trust_provider.rs");
}
pub mod verifiedpermissions {
    include!("resources/verifiedpermissions/identity_source.rs");
    include!("resources/verifiedpermissions/policy.rs");
    include!("resources/verifiedpermissions/policy_store.rs");
    include!("resources/verifiedpermissions/policy_template.rs");
    include!("resources/verifiedpermissions/schema.rs");
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
    pub mod ssoadmin {
        include!("functions/ssoadmin/get_application.rs");
        include!("functions/ssoadmin/get_application_assignments.rs");
        include!("functions/ssoadmin/get_application_providers.rs");
        include!("functions/ssoadmin/get_instances.rs");
        include!("functions/ssoadmin/get_permission_set.rs");
        include!("functions/ssoadmin/get_permission_sets.rs");
        include!("functions/ssoadmin/get_principal_application_assignments.rs");
    }
    pub mod storagegateway {
        include!("functions/storagegateway/get_local_disk.rs");
    }
    pub mod synthetics {
        include!("functions/synthetics/get_runtime_version.rs");
        include!("functions/synthetics/get_runtime_versions.rs");
    }
    pub mod timestreamwrite {
        include!("functions/timestreamwrite/get_database.rs");
        include!("functions/timestreamwrite/get_table.rs");
    }
    pub mod transfer {
        include!("functions/transfer/get_connector.rs");
        include!("functions/transfer/get_server.rs");
    }
    pub mod verifiedpermissions {
        include!("functions/verifiedpermissions/get_policy_store.rs");
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
    pub mod ssoadmin {
        include!("types/ssoadmin/application_portal_options.rs");
        include!("types/ssoadmin/application_portal_options_sign_in_options.rs");
        include!(
            "types/ssoadmin/customer_managed_policy_attachment_customer_managed_policy_reference.rs"
        );
        include!("types/ssoadmin/instance_access_control_attributes_attribute.rs");
        include!("types/ssoadmin/instance_access_control_attributes_attribute_value.rs");
        include!(
            "types/ssoadmin/permissions_boundary_attachment_permissions_boundary.rs"
        );
        include!(
            "types/ssoadmin/permissions_boundary_attachment_permissions_boundary_customer_managed_policy_reference.rs"
        );
        include!(
            "types/ssoadmin/trusted_token_issuer_trusted_token_issuer_configuration.rs"
        );
        include!(
            "types/ssoadmin/trusted_token_issuer_trusted_token_issuer_configuration_oidc_jwt_configuration.rs"
        );
        include!("types/ssoadmin/get_application_assignments_application_assignment.rs");
        include!("types/ssoadmin/get_application_portal_option.rs");
        include!("types/ssoadmin/get_application_portal_option_sign_in_option.rs");
        include!("types/ssoadmin/get_application_providers_application_provider.rs");
        include!(
            "types/ssoadmin/get_application_providers_application_provider_display_data.rs"
        );
        include!(
            "types/ssoadmin/get_principal_application_assignments_application_assignment.rs"
        );
    }
    pub mod storagegateway {
        include!("types/storagegateway/file_system_association_cache_attributes.rs");
        include!("types/storagegateway/gateway_gateway_network_interface.rs");
        include!("types/storagegateway/gateway_maintenance_start_time.rs");
        include!("types/storagegateway/gateway_smb_active_directory_settings.rs");
        include!("types/storagegateway/nfs_file_share_cache_attributes.rs");
        include!("types/storagegateway/nfs_file_share_nfs_file_share_defaults.rs");
        include!("types/storagegateway/smb_file_share_cache_attributes.rs");
    }
    pub mod synthetics {
        include!("types/synthetics/canary_artifact_config.rs");
        include!("types/synthetics/canary_artifact_config_s_3_encryption.rs");
        include!("types/synthetics/canary_run_config.rs");
        include!("types/synthetics/canary_schedule.rs");
        include!("types/synthetics/canary_timeline.rs");
        include!("types/synthetics/canary_vpc_config.rs");
        include!("types/synthetics/get_runtime_versions_runtime_version.rs");
    }
    pub mod timestreaminfluxdb {
        include!("types/timestreaminfluxdb/db_instance_log_delivery_configuration.rs");
        include!(
            "types/timestreaminfluxdb/db_instance_log_delivery_configuration_s_3_configuration.rs"
        );
        include!("types/timestreaminfluxdb/db_instance_timeouts.rs");
    }
    pub mod timestreamwrite {
        include!("types/timestreamwrite/table_magnetic_store_write_properties.rs");
        include!(
            "types/timestreamwrite/table_magnetic_store_write_properties_magnetic_store_rejected_data_location.rs"
        );
        include!(
            "types/timestreamwrite/table_magnetic_store_write_properties_magnetic_store_rejected_data_location_s_3_configuration.rs"
        );
        include!("types/timestreamwrite/table_retention_properties.rs");
        include!("types/timestreamwrite/table_schema.rs");
        include!("types/timestreamwrite/table_schema_composite_partition_key.rs");
        include!("types/timestreamwrite/get_table_magnetic_store_write_property.rs");
        include!(
            "types/timestreamwrite/get_table_magnetic_store_write_property_magnetic_store_rejected_data_location.rs"
        );
        include!(
            "types/timestreamwrite/get_table_magnetic_store_write_property_magnetic_store_rejected_data_location_s_3_configuration.rs"
        );
        include!("types/timestreamwrite/get_table_retention_property.rs");
        include!("types/timestreamwrite/get_table_schema.rs");
        include!("types/timestreamwrite/get_table_schema_composite_partition_key.rs");
    }
    pub mod transcribe {
        include!("types/transcribe/language_model_input_data_config.rs");
    }
    pub mod transfer {
        include!("types/transfer/access_home_directory_mapping.rs");
        include!("types/transfer/access_posix_profile.rs");
        include!("types/transfer/connector_as_2_config.rs");
        include!("types/transfer/connector_sftp_config.rs");
        include!("types/transfer/server_endpoint_details.rs");
        include!("types/transfer/server_protocol_details.rs");
        include!("types/transfer/server_s_3_storage_options.rs");
        include!("types/transfer/server_workflow_details.rs");
        include!("types/transfer/server_workflow_details_on_partial_upload.rs");
        include!("types/transfer/server_workflow_details_on_upload.rs");
        include!("types/transfer/user_home_directory_mapping.rs");
        include!("types/transfer/user_posix_profile.rs");
        include!("types/transfer/workflow_on_exception_step.rs");
        include!("types/transfer/workflow_on_exception_step_copy_step_details.rs");
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_copy_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_on_exception_step_custom_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_decrypt_step_details.rs");
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_on_exception_step_decrypt_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_on_exception_step_delete_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_tag_step_details.rs");
        include!("types/transfer/workflow_on_exception_step_tag_step_details_tag.rs");
        include!("types/transfer/workflow_step.rs");
        include!("types/transfer/workflow_step_copy_step_details.rs");
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_copy_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_step_custom_step_details.rs");
        include!("types/transfer/workflow_step_decrypt_step_details.rs");
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location_efs_file_location.rs"
        );
        include!(
            "types/transfer/workflow_step_decrypt_step_details_destination_file_location_s_3_file_location.rs"
        );
        include!("types/transfer/workflow_step_delete_step_details.rs");
        include!("types/transfer/workflow_step_tag_step_details.rs");
        include!("types/transfer/workflow_step_tag_step_details_tag.rs");
        include!("types/transfer/get_connector_as_2_config.rs");
        include!("types/transfer/get_connector_sftp_config.rs");
    }
    pub mod verifiedaccess {
        include!("types/verifiedaccess/endpoint_load_balancer_options.rs");
        include!("types/verifiedaccess/endpoint_network_interface_options.rs");
        include!("types/verifiedaccess/endpoint_sse_specification.rs");
        include!("types/verifiedaccess/group_sse_configuration.rs");
        include!("types/verifiedaccess/instance_logging_configuration_access_logs.rs");
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_cloudwatch_logs.rs"
        );
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_kinesis_data_firehose.rs"
        );
        include!(
            "types/verifiedaccess/instance_logging_configuration_access_logs_s_3.rs"
        );
        include!("types/verifiedaccess/instance_verified_access_trust_provider.rs");
        include!("types/verifiedaccess/trust_provider_device_options.rs");
        include!("types/verifiedaccess/trust_provider_oidc_options.rs");
    }
    pub mod verifiedpermissions {
        include!("types/verifiedpermissions/identity_source_configuration.rs");
        include!(
            "types/verifiedpermissions/identity_source_configuration_cognito_user_pool_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_cognito_user_pool_configuration_group_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_group_configuration.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection_access_token_only.rs"
        );
        include!(
            "types/verifiedpermissions/identity_source_configuration_open_id_connect_configuration_token_selection_identity_token_only.rs"
        );
        include!("types/verifiedpermissions/policy_definition.rs");
        include!("types/verifiedpermissions/policy_definition_static.rs");
        include!("types/verifiedpermissions/policy_definition_template_linked.rs");
        include!(
            "types/verifiedpermissions/policy_definition_template_linked_principal.rs"
        );
        include!(
            "types/verifiedpermissions/policy_definition_template_linked_resource.rs"
        );
        include!("types/verifiedpermissions/policy_store_validation_settings.rs");
        include!("types/verifiedpermissions/schema_definition.rs");
        include!("types/verifiedpermissions/get_policy_store_validation_setting.rs");
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
