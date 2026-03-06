pub mod paymentcryptography {
    include!("resources/paymentcryptography/key.rs");
    include!("resources/paymentcryptography/key_alias.rs");
}
pub mod pinpoint {
    include!("resources/pinpoint/adm_channel.rs");
    include!("resources/pinpoint/apns_channel.rs");
    include!("resources/pinpoint/apns_sandbox_channel.rs");
    include!("resources/pinpoint/apns_voip_channel.rs");
    include!("resources/pinpoint/apns_voip_sandbox_channel.rs");
    include!("resources/pinpoint/app.rs");
    include!("resources/pinpoint/baidu_channel.rs");
    include!("resources/pinpoint/email_channel.rs");
    include!("resources/pinpoint/email_template.rs");
    include!("resources/pinpoint/event_stream.rs");
    include!("resources/pinpoint/gcm_channel.rs");
    include!("resources/pinpoint/sms_channel.rs");
    include!("resources/pinpoint/smsvoicev_2_configuration_set.rs");
    include!("resources/pinpoint/smsvoicev_2_opt_out_list.rs");
    include!("resources/pinpoint/smsvoicev_2_phone_number.rs");
}
pub mod pipes {
    include!("resources/pipes/pipe.rs");
}
pub mod qldb {
    include!("resources/qldb/ledger.rs");
    include!("resources/qldb/stream.rs");
}
pub mod quicksight {
    include!("resources/quicksight/account_subscription.rs");
    include!("resources/quicksight/analysis.rs");
    include!("resources/quicksight/dashboard.rs");
    include!("resources/quicksight/data_set.rs");
    include!("resources/quicksight/data_source.rs");
    include!("resources/quicksight/folder.rs");
    include!("resources/quicksight/folder_membership.rs");
    include!("resources/quicksight/group.rs");
    include!("resources/quicksight/group_membership.rs");
    include!("resources/quicksight/iam_policy_assignment.rs");
    include!("resources/quicksight/ingestion.rs");
    include!("resources/quicksight/namespace.rs");
    include!("resources/quicksight/refresh_schedule.rs");
    include!("resources/quicksight/template.rs");
    include!("resources/quicksight/template_alias.rs");
    include!("resources/quicksight/theme.rs");
    include!("resources/quicksight/user.rs");
    include!("resources/quicksight/vpc_connection.rs");
}
pub mod ram {
    include!("resources/ram/principal_association.rs");
    include!("resources/ram/resource_association.rs");
    include!("resources/ram/resource_share.rs");
    include!("resources/ram/resource_share_accepter.rs");
    include!("resources/ram/sharing_with_organization.rs");
}
pub mod rbin {
    include!("resources/rbin/rule.rs");
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
    pub mod outposts {
        include!("functions/outposts/get_asset.rs");
        include!("functions/outposts/get_assets.rs");
        include!("functions/outposts/get_outpost.rs");
        include!("functions/outposts/get_outpost_instance_type.rs");
        include!("functions/outposts/get_outpost_instance_types.rs");
        include!("functions/outposts/get_outposts.rs");
        include!("functions/outposts/get_site.rs");
        include!("functions/outposts/get_sites.rs");
    }
    pub mod polly {
        include!("functions/polly/get_voices.rs");
    }
    pub mod pricing {
        include!("functions/pricing/get_product.rs");
    }
    pub mod qldb {
        include!("functions/qldb/get_ledger.rs");
    }
    pub mod quicksight {
        include!("functions/quicksight/get_analysis.rs");
        include!("functions/quicksight/get_data_set.rs");
        include!("functions/quicksight/get_quicksight_analysis.rs");
        include!("functions/quicksight/get_quicksight_group.rs");
        include!("functions/quicksight/get_quicksight_user.rs");
        include!("functions/quicksight/get_theme.rs");
    }
    pub mod ram {
        include!("functions/ram/get_resource_share.rs");
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
    pub mod paymentcryptography {
        include!("types/paymentcryptography/key_key_attributes.rs");
        include!("types/paymentcryptography/key_key_attributes_key_modes_of_use.rs");
        include!("types/paymentcryptography/key_timeouts.rs");
    }
    pub mod pinpoint {
        include!("types/pinpoint/app_campaign_hook.rs");
        include!("types/pinpoint/app_limits.rs");
        include!("types/pinpoint/app_quiet_time.rs");
        include!("types/pinpoint/email_template_email_template.rs");
        include!("types/pinpoint/email_template_email_template_header.rs");
        include!("types/pinpoint/smsvoicev_2_phone_number_timeouts.rs");
    }
    pub mod pipes {
        include!("types/pipes/pipe_enrichment_parameters.rs");
        include!("types/pipes/pipe_enrichment_parameters_http_parameters.rs");
        include!("types/pipes/pipe_log_configuration.rs");
        include!(
            "types/pipes/pipe_log_configuration_cloudwatch_logs_log_destination.rs"
        );
        include!("types/pipes/pipe_log_configuration_firehose_log_destination.rs");
        include!("types/pipes/pipe_log_configuration_s_3_log_destination.rs");
        include!("types/pipes/pipe_source_parameters.rs");
        include!("types/pipes/pipe_source_parameters_activemq_broker_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_activemq_broker_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_dynamodb_stream_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_dynamodb_stream_parameters_dead_letter_config.rs"
        );
        include!("types/pipes/pipe_source_parameters_filter_criteria.rs");
        include!("types/pipes/pipe_source_parameters_filter_criteria_filter.rs");
        include!("types/pipes/pipe_source_parameters_kinesis_stream_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_kinesis_stream_parameters_dead_letter_config.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_managed_streaming_kafka_parameters.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_managed_streaming_kafka_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_rabbitmq_broker_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_rabbitmq_broker_parameters_credentials.rs"
        );
        include!("types/pipes/pipe_source_parameters_self_managed_kafka_parameters.rs");
        include!(
            "types/pipes/pipe_source_parameters_self_managed_kafka_parameters_credentials.rs"
        );
        include!(
            "types/pipes/pipe_source_parameters_self_managed_kafka_parameters_vpc.rs"
        );
        include!("types/pipes/pipe_source_parameters_sqs_queue_parameters.rs");
        include!("types/pipes/pipe_target_parameters.rs");
        include!("types/pipes/pipe_target_parameters_batch_job_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_array_properties.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides_environment.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_container_overrides_resource_requirement.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_depends_on.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_batch_job_parameters_retry_strategy.rs"
        );
        include!("types/pipes/pipe_target_parameters_cloudwatch_logs_parameters.rs");
        include!("types/pipes/pipe_target_parameters_ecs_task_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_capacity_provider_strategy.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_network_configuration.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_network_configuration_aws_vpc_configuration.rs"
        );
        include!("types/pipes/pipe_target_parameters_ecs_task_parameters_overrides.rs");
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_environment.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_environment_file.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_container_override_resource_requirement.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_ephemeral_storage.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_overrides_inference_accelerator_override.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_placement_constraint.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_ecs_task_parameters_placement_strategy.rs"
        );
        include!(
            "types/pipes/pipe_target_parameters_eventbridge_event_bus_parameters.rs"
        );
        include!("types/pipes/pipe_target_parameters_http_parameters.rs");
        include!("types/pipes/pipe_target_parameters_kinesis_stream_parameters.rs");
        include!("types/pipes/pipe_target_parameters_lambda_function_parameters.rs");
        include!("types/pipes/pipe_target_parameters_redshift_data_parameters.rs");
        include!("types/pipes/pipe_target_parameters_sagemaker_pipeline_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_sagemaker_pipeline_parameters_pipeline_parameter.rs"
        );
        include!("types/pipes/pipe_target_parameters_sqs_queue_parameters.rs");
        include!(
            "types/pipes/pipe_target_parameters_step_function_state_machine_parameters.rs"
        );
    }
    pub mod polly {
        include!("types/polly/get_voices_voice.rs");
    }
    pub mod pricing {
        include!("types/pricing/get_product_filter.rs");
    }
    pub mod qldb {
        include!("types/qldb/stream_kinesis_configuration.rs");
    }
    pub mod quicksight {
        include!("types/quicksight/analysis_parameters.rs");
        include!("types/quicksight/analysis_parameters_date_time_parameter.rs");
        include!("types/quicksight/analysis_parameters_decimal_parameter.rs");
        include!("types/quicksight/analysis_parameters_integer_parameter.rs");
        include!("types/quicksight/analysis_parameters_string_parameter.rs");
        include!("types/quicksight/analysis_permission.rs");
        include!("types/quicksight/analysis_source_entity.rs");
        include!("types/quicksight/analysis_source_entity_source_template.rs");
        include!(
            "types/quicksight/analysis_source_entity_source_template_data_set_reference.rs"
        );
        include!("types/quicksight/dashboard_dashboard_publish_options.rs");
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_ad_hoc_filtering_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_drill_up_down_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_menu_label_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_data_point_tooltip_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_export_to_csv_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_export_with_hidden_fields_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_sheet_controls_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_sheet_layout_element_maximization_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_visual_axis_sort_option.rs"
        );
        include!(
            "types/quicksight/dashboard_dashboard_publish_options_visual_menu_option.rs"
        );
        include!("types/quicksight/dashboard_parameters.rs");
        include!("types/quicksight/dashboard_parameters_date_time_parameter.rs");
        include!("types/quicksight/dashboard_parameters_decimal_parameter.rs");
        include!("types/quicksight/dashboard_parameters_integer_parameter.rs");
        include!("types/quicksight/dashboard_parameters_string_parameter.rs");
        include!("types/quicksight/dashboard_permission.rs");
        include!("types/quicksight/dashboard_source_entity.rs");
        include!("types/quicksight/dashboard_source_entity_source_template.rs");
        include!(
            "types/quicksight/dashboard_source_entity_source_template_data_set_reference.rs"
        );
        include!("types/quicksight/data_set_column_group.rs");
        include!("types/quicksight/data_set_column_group_geo_spatial_column_group.rs");
        include!("types/quicksight/data_set_column_level_permission_rule.rs");
        include!("types/quicksight/data_set_data_set_usage_configuration.rs");
        include!("types/quicksight/data_set_field_folder.rs");
        include!("types/quicksight/data_set_logical_table_map.rs");
        include!("types/quicksight/data_set_logical_table_map_data_transform.rs");
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_cast_column_type_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_create_columns_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_create_columns_operation_column.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_filter_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_project_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_rename_column_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation_tag.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_tag_column_operation_tag_column_description.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_data_transform_untag_column_operation.rs"
        );
        include!("types/quicksight/data_set_logical_table_map_source.rs");
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction_left_join_key_properties.rs"
        );
        include!(
            "types/quicksight/data_set_logical_table_map_source_join_instruction_right_join_key_properties.rs"
        );
        include!("types/quicksight/data_set_output_column.rs");
        include!("types/quicksight/data_set_permission.rs");
        include!("types/quicksight/data_set_physical_table_map.rs");
        include!("types/quicksight/data_set_physical_table_map_custom_sql.rs");
        include!("types/quicksight/data_set_physical_table_map_custom_sql_column.rs");
        include!("types/quicksight/data_set_physical_table_map_relational_table.rs");
        include!(
            "types/quicksight/data_set_physical_table_map_relational_table_input_column.rs"
        );
        include!("types/quicksight/data_set_physical_table_map_s_3_source.rs");
        include!(
            "types/quicksight/data_set_physical_table_map_s_3_source_input_column.rs"
        );
        include!(
            "types/quicksight/data_set_physical_table_map_s_3_source_upload_settings.rs"
        );
        include!("types/quicksight/data_set_refresh_properties.rs");
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration.rs"
        );
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration_incremental_refresh.rs"
        );
        include!(
            "types/quicksight/data_set_refresh_properties_refresh_configuration_incremental_refresh_lookback_window.rs"
        );
        include!("types/quicksight/data_set_row_level_permission_data_set.rs");
        include!("types/quicksight/data_set_row_level_permission_tag_configuration.rs");
        include!(
            "types/quicksight/data_set_row_level_permission_tag_configuration_tag_rule.rs"
        );
        include!("types/quicksight/data_source_credentials.rs");
        include!("types/quicksight/data_source_credentials_credential_pair.rs");
        include!("types/quicksight/data_source_parameters.rs");
        include!("types/quicksight/data_source_parameters_amazon_elasticsearch.rs");
        include!("types/quicksight/data_source_parameters_athena.rs");
        include!("types/quicksight/data_source_parameters_aurora.rs");
        include!("types/quicksight/data_source_parameters_aurora_postgresql.rs");
        include!("types/quicksight/data_source_parameters_aws_iot_analytics.rs");
        include!("types/quicksight/data_source_parameters_databricks.rs");
        include!("types/quicksight/data_source_parameters_jira.rs");
        include!("types/quicksight/data_source_parameters_maria_db.rs");
        include!("types/quicksight/data_source_parameters_mysql.rs");
        include!("types/quicksight/data_source_parameters_oracle.rs");
        include!("types/quicksight/data_source_parameters_postgresql.rs");
        include!("types/quicksight/data_source_parameters_presto.rs");
        include!("types/quicksight/data_source_parameters_rds.rs");
        include!("types/quicksight/data_source_parameters_redshift.rs");
        include!("types/quicksight/data_source_parameters_s_3.rs");
        include!(
            "types/quicksight/data_source_parameters_s_3_manifest_file_location.rs"
        );
        include!("types/quicksight/data_source_parameters_service_now.rs");
        include!("types/quicksight/data_source_parameters_snowflake.rs");
        include!("types/quicksight/data_source_parameters_spark.rs");
        include!("types/quicksight/data_source_parameters_sql_server.rs");
        include!("types/quicksight/data_source_parameters_teradata.rs");
        include!("types/quicksight/data_source_parameters_twitter.rs");
        include!("types/quicksight/data_source_permission.rs");
        include!("types/quicksight/data_source_ssl_properties.rs");
        include!("types/quicksight/data_source_vpc_connection_properties.rs");
        include!("types/quicksight/folder_permission.rs");
        include!("types/quicksight/iam_policy_assignment_identities.rs");
        include!("types/quicksight/namespace_timeouts.rs");
        include!("types/quicksight/refresh_schedule_schedule.rs");
        include!("types/quicksight/refresh_schedule_schedule_schedule_frequency.rs");
        include!(
            "types/quicksight/refresh_schedule_schedule_schedule_frequency_refresh_on_day.rs"
        );
        include!("types/quicksight/template_permission.rs");
        include!("types/quicksight/template_source_entity.rs");
        include!("types/quicksight/template_source_entity_source_analysis.rs");
        include!(
            "types/quicksight/template_source_entity_source_analysis_data_set_reference.rs"
        );
        include!("types/quicksight/template_source_entity_source_template.rs");
        include!("types/quicksight/theme_configuration.rs");
        include!("types/quicksight/theme_configuration_data_color_palette.rs");
        include!("types/quicksight/theme_configuration_sheet.rs");
        include!("types/quicksight/theme_configuration_sheet_tile.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_border.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout_gutter.rs");
        include!("types/quicksight/theme_configuration_sheet_tile_layout_margin.rs");
        include!("types/quicksight/theme_configuration_typography.rs");
        include!("types/quicksight/theme_configuration_typography_font_family.rs");
        include!("types/quicksight/theme_configuration_ui_color_palette.rs");
        include!("types/quicksight/theme_permission.rs");
        include!("types/quicksight/vpc_connection_timeouts.rs");
        include!("types/quicksight/get_analysis_permission.rs");
        include!("types/quicksight/get_data_set_column_group.rs");
        include!(
            "types/quicksight/get_data_set_column_group_geo_spatial_column_group.rs"
        );
        include!("types/quicksight/get_data_set_column_level_permission_rule.rs");
        include!("types/quicksight/get_data_set_data_set_usage_configuration.rs");
        include!("types/quicksight/get_data_set_field_folder.rs");
        include!("types/quicksight/get_data_set_logical_table_map.rs");
        include!("types/quicksight/get_data_set_logical_table_map_data_transform.rs");
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_cast_column_type_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_create_columns_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_create_columns_operation_column.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_filter_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_project_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_rename_column_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation_tag.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_tag_column_operation_tag_column_description.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_data_transform_untag_column_operation.rs"
        );
        include!("types/quicksight/get_data_set_logical_table_map_source.rs");
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction_left_join_key_property.rs"
        );
        include!(
            "types/quicksight/get_data_set_logical_table_map_source_join_instruction_right_join_key_property.rs"
        );
        include!("types/quicksight/get_data_set_permission.rs");
        include!("types/quicksight/get_data_set_physical_table_map.rs");
        include!("types/quicksight/get_data_set_physical_table_map_custom_sql.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_custom_sql_column.rs"
        );
        include!("types/quicksight/get_data_set_physical_table_map_relational_table.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_relational_table_input_column.rs"
        );
        include!("types/quicksight/get_data_set_physical_table_map_s_3_source.rs");
        include!(
            "types/quicksight/get_data_set_physical_table_map_s_3_source_input_column.rs"
        );
        include!(
            "types/quicksight/get_data_set_physical_table_map_s_3_source_upload_setting.rs"
        );
        include!("types/quicksight/get_data_set_row_level_permission_data_set.rs");
        include!(
            "types/quicksight/get_data_set_row_level_permission_tag_configuration.rs"
        );
        include!(
            "types/quicksight/get_data_set_row_level_permission_tag_configuration_tag_rule.rs"
        );
        include!("types/quicksight/get_quicksight_analysis_permission.rs");
        include!("types/quicksight/get_theme_configuration.rs");
        include!("types/quicksight/get_theme_configuration_data_color_palette.rs");
        include!("types/quicksight/get_theme_configuration_sheet.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_border.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout_gutter.rs");
        include!("types/quicksight/get_theme_configuration_sheet_tile_layout_margin.rs");
        include!("types/quicksight/get_theme_configuration_typography.rs");
        include!("types/quicksight/get_theme_configuration_typography_font_family.rs");
        include!("types/quicksight/get_theme_configuration_ui_color_palette.rs");
        include!("types/quicksight/get_theme_permission.rs");
    }
    pub mod ram {
        include!("types/ram/get_resource_share_filter.rs");
    }
    pub mod rbin {
        include!("types/rbin/rule_lock_configuration.rs");
        include!("types/rbin/rule_lock_configuration_unlock_delay.rs");
        include!("types/rbin/rule_resource_tag.rs");
        include!("types/rbin/rule_retention_period.rs");
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
