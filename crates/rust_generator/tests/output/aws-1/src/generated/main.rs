pub mod appconfig {
    include!("resources/appconfig/application.rs");
    include!("resources/appconfig/configuration_profile.rs");
    include!("resources/appconfig/deployment.rs");
    include!("resources/appconfig/deployment_strategy.rs");
    include!("resources/appconfig/environment.rs");
    include!("resources/appconfig/event_integration.rs");
    include!("resources/appconfig/extension.rs");
    include!("resources/appconfig/extension_association.rs");
    include!("resources/appconfig/hosted_configuration_version.rs");
}
pub mod appfabric {
    include!("resources/appfabric/app_authorization.rs");
    include!("resources/appfabric/app_authorization_connection.rs");
    include!("resources/appfabric/app_bundle.rs");
    include!("resources/appfabric/ingestion.rs");
    include!("resources/appfabric/ingestion_destination.rs");
}
pub mod appflow {
    include!("resources/appflow/connector_profile.rs");
    include!("resources/appflow/flow.rs");
}
pub mod appintegrations {
    include!("resources/appintegrations/data_integration.rs");
}
pub mod applicationinsights {
    include!("resources/applicationinsights/application.rs");
}
pub mod appmesh {
    include!("resources/appmesh/gateway_route.rs");
    include!("resources/appmesh/mesh.rs");
    include!("resources/appmesh/route.rs");
    include!("resources/appmesh/virtual_gateway.rs");
    include!("resources/appmesh/virtual_node.rs");
    include!("resources/appmesh/virtual_router.rs");
    include!("resources/appmesh/virtual_service.rs");
}
pub mod apprunner {
    include!("resources/apprunner/auto_scaling_configuration_version.rs");
    include!("resources/apprunner/connection.rs");
    include!("resources/apprunner/custom_domain_association.rs");
    include!("resources/apprunner/default_auto_scaling_configuration_version.rs");
    include!("resources/apprunner/deployment.rs");
    include!("resources/apprunner/observability_configuration.rs");
    include!("resources/apprunner/service.rs");
    include!("resources/apprunner/vpc_connector.rs");
    include!("resources/apprunner/vpc_ingress_connection.rs");
}
pub mod appstream {
    include!("resources/appstream/directory_config.rs");
    include!("resources/appstream/fleet.rs");
    include!("resources/appstream/fleet_stack_association.rs");
    include!("resources/appstream/image_builder.rs");
    include!("resources/appstream/stack.rs");
    include!("resources/appstream/user.rs");
    include!("resources/appstream/user_stack_association.rs");
}
pub mod appsync {
    include!("resources/appsync/api_cache.rs");
    include!("resources/appsync/api_key.rs");
    include!("resources/appsync/data_source.rs");
    include!("resources/appsync/domain_name.rs");
    include!("resources/appsync/domain_name_api_association.rs");
    include!("resources/appsync/function.rs");
    include!("resources/appsync/graph_ql_api.rs");
    include!("resources/appsync/resolver.rs");
    include!("resources/appsync/source_api_association.rs");
    include!("resources/appsync/type.rs");
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
    pub mod appconfig {
        include!("functions/appconfig/get_configuration_profile.rs");
        include!("functions/appconfig/get_configuration_profiles.rs");
        include!("functions/appconfig/get_environment.rs");
        include!("functions/appconfig/get_environments.rs");
    }
    pub mod appintegrations {
        include!("functions/appintegrations/get_event_integration.rs");
    }
    pub mod appmesh {
        include!("functions/appmesh/get_gateway_route.rs");
        include!("functions/appmesh/get_mesh.rs");
        include!("functions/appmesh/get_route.rs");
        include!("functions/appmesh/get_virtual_gateway.rs");
        include!("functions/appmesh/get_virtual_node.rs");
        include!("functions/appmesh/get_virtual_router.rs");
        include!("functions/appmesh/get_virtual_service.rs");
    }
    pub mod apprunner {
        include!("functions/apprunner/get_hosted_zone_id.rs");
    }
    pub mod appstream {
        include!("functions/appstream/get_image.rs");
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
    pub mod appconfig {
        include!("types/appconfig/configuration_profile_validator.rs");
        include!("types/appconfig/environment_monitor.rs");
        include!("types/appconfig/event_integration_event_filter.rs");
        include!("types/appconfig/extension_action_point.rs");
        include!("types/appconfig/extension_action_point_action.rs");
        include!("types/appconfig/extension_parameter.rs");
        include!("types/appconfig/get_configuration_profile_validator.rs");
        include!("types/appconfig/get_environment_monitor.rs");
    }
    pub mod appfabric {
        include!("types/appfabric/app_authorization_connection_auth_request.rs");
        include!("types/appfabric/app_authorization_connection_tenant.rs");
        include!("types/appfabric/app_authorization_connection_timeouts.rs");
        include!("types/appfabric/app_authorization_credential.rs");
        include!("types/appfabric/app_authorization_credential_api_key_credential.rs");
        include!("types/appfabric/app_authorization_credential_oauth_2_credential.rs");
        include!("types/appfabric/app_authorization_tenant.rs");
        include!("types/appfabric/app_authorization_timeouts.rs");
        include!("types/appfabric/ingestion_destination_destination_configuration.rs");
        include!(
            "types/appfabric/ingestion_destination_destination_configuration_audit_log.rs"
        );
        include!(
            "types/appfabric/ingestion_destination_destination_configuration_audit_log_destination.rs"
        );
        include!(
            "types/appfabric/ingestion_destination_destination_configuration_audit_log_destination_firehose_stream.rs"
        );
        include!(
            "types/appfabric/ingestion_destination_destination_configuration_audit_log_destination_s_3_bucket.rs"
        );
        include!("types/appfabric/ingestion_destination_processing_configuration.rs");
        include!(
            "types/appfabric/ingestion_destination_processing_configuration_audit_log.rs"
        );
        include!("types/appfabric/ingestion_destination_timeouts.rs");
    }
    pub mod appflow {
        include!("types/appflow/connector_profile_connector_profile_config.rs");
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_amplitude.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector_api_key.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector_basic.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector_custom.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector_oauth_2.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_custom_connector_oauth_2_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_datadog.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_dynatrace.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_google_analytics.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_google_analytics_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_honeycode.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_honeycode_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_infor_nexus.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_marketo.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_marketo_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_redshift.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_salesforce.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_salesforce_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_sapo_data.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_sapo_data_basic_auth_credentials.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_sapo_data_oauth_credentials.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_sapo_data_oauth_credentials_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_service_now.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_singular.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_slack.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_slack_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_snowflake.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_trendmicro.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_veeva.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_zendesk.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_credentials_zendesk_oauth_request.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_amplitude.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_custom_connector.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_custom_connector_oauth_2_properties.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_datadog.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_dynatrace.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_google_analytics.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_honeycode.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_infor_nexus.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_marketo.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_redshift.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_salesforce.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_sapo_data.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_sapo_data_oauth_properties.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_service_now.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_singular.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_slack.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_snowflake.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_trendmicro.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_veeva.rs"
        );
        include!(
            "types/appflow/connector_profile_connector_profile_config_connector_profile_properties_zendesk.rs"
        );
        include!("types/appflow/flow_destination_flow_config.rs");
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_custom_connector.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_custom_connector_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_customer_profiles.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_event_bridge.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_event_bridge_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_honeycode.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_honeycode_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_lookout_metrics.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_marketo.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_marketo_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_redshift.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_redshift_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_s_3.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_s_3_s_3_output_format_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_s_3_s_3_output_format_config_aggregation_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_s_3_s_3_output_format_config_prefix_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_salesforce.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_salesforce_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_sapo_data.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_sapo_data_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_sapo_data_success_response_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_snowflake.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_snowflake_error_handling_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_upsolver.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_upsolver_s_3_output_format_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_upsolver_s_3_output_format_config_aggregation_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_upsolver_s_3_output_format_config_prefix_config.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_zendesk.rs"
        );
        include!(
            "types/appflow/flow_destination_flow_config_destination_connector_properties_zendesk_error_handling_config.rs"
        );
        include!("types/appflow/flow_metadata_catalog_config.rs");
        include!("types/appflow/flow_metadata_catalog_config_glue_data_catalog.rs");
        include!("types/appflow/flow_source_flow_config.rs");
        include!("types/appflow/flow_source_flow_config_incremental_pull_config.rs");
        include!("types/appflow/flow_source_flow_config_source_connector_properties.rs");
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_amplitude.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_custom_connector.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_datadog.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_dynatrace.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_google_analytics.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_infor_nexus.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_marketo.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_s_3.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_s_3_s_3_input_format_config.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_salesforce.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_sapo_data.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_sapo_data_pagination_config.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_sapo_data_parallelism_config.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_service_now.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_singular.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_slack.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_trendmicro.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_veeva.rs"
        );
        include!(
            "types/appflow/flow_source_flow_config_source_connector_properties_zendesk.rs"
        );
        include!("types/appflow/flow_task.rs");
        include!("types/appflow/flow_task_connector_operator.rs");
        include!("types/appflow/flow_trigger_config.rs");
        include!("types/appflow/flow_trigger_config_trigger_properties.rs");
        include!("types/appflow/flow_trigger_config_trigger_properties_scheduled.rs");
    }
    pub mod appintegrations {
        include!("types/appintegrations/data_integration_schedule_config.rs");
        include!("types/appintegrations/get_event_integration_event_filter.rs");
    }
    pub mod appmesh {
        include!("types/appmesh/gateway_route_spec.rs");
        include!("types/appmesh/gateway_route_spec_grpc_route.rs");
        include!("types/appmesh/gateway_route_spec_grpc_route_action.rs");
        include!("types/appmesh/gateway_route_spec_grpc_route_action_target.rs");
        include!(
            "types/appmesh/gateway_route_spec_grpc_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/gateway_route_spec_grpc_route_match.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route_action.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route_action_rewrite.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_action_rewrite_hostname.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_2_route_action_rewrite_path.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_action_rewrite_prefix.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_2_route_action_target.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_2_route_match.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route_match_header.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route_match_header_match.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_match_header_match_range.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_2_route_match_hostname.rs");
        include!("types/appmesh/gateway_route_spec_http_2_route_match_path.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_match_query_parameter.rs"
        );
        include!(
            "types/appmesh/gateway_route_spec_http_2_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_route.rs");
        include!("types/appmesh/gateway_route_spec_http_route_action.rs");
        include!("types/appmesh/gateway_route_spec_http_route_action_rewrite.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_route_action_rewrite_hostname.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_route_action_rewrite_path.rs");
        include!("types/appmesh/gateway_route_spec_http_route_action_rewrite_prefix.rs");
        include!("types/appmesh/gateway_route_spec_http_route_action_target.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_route_match.rs");
        include!("types/appmesh/gateway_route_spec_http_route_match_header.rs");
        include!("types/appmesh/gateway_route_spec_http_route_match_header_match.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_route_match_header_match_range.rs"
        );
        include!("types/appmesh/gateway_route_spec_http_route_match_hostname.rs");
        include!("types/appmesh/gateway_route_spec_http_route_match_path.rs");
        include!("types/appmesh/gateway_route_spec_http_route_match_query_parameter.rs");
        include!(
            "types/appmesh/gateway_route_spec_http_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/mesh_spec.rs");
        include!("types/appmesh/mesh_spec_egress_filter.rs");
        include!("types/appmesh/mesh_spec_service_discovery.rs");
        include!("types/appmesh/route_spec.rs");
        include!("types/appmesh/route_spec_grpc_route.rs");
        include!("types/appmesh/route_spec_grpc_route_action.rs");
        include!("types/appmesh/route_spec_grpc_route_action_weighted_target.rs");
        include!("types/appmesh/route_spec_grpc_route_match.rs");
        include!("types/appmesh/route_spec_grpc_route_match_metadata.rs");
        include!("types/appmesh/route_spec_grpc_route_match_metadata_match.rs");
        include!("types/appmesh/route_spec_grpc_route_match_metadata_match_range.rs");
        include!("types/appmesh/route_spec_grpc_route_retry_policy.rs");
        include!(
            "types/appmesh/route_spec_grpc_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/route_spec_grpc_route_timeout.rs");
        include!("types/appmesh/route_spec_grpc_route_timeout_idle.rs");
        include!("types/appmesh/route_spec_grpc_route_timeout_per_request.rs");
        include!("types/appmesh/route_spec_http_2_route.rs");
        include!("types/appmesh/route_spec_http_2_route_action.rs");
        include!("types/appmesh/route_spec_http_2_route_action_weighted_target.rs");
        include!("types/appmesh/route_spec_http_2_route_match.rs");
        include!("types/appmesh/route_spec_http_2_route_match_header.rs");
        include!("types/appmesh/route_spec_http_2_route_match_header_match.rs");
        include!("types/appmesh/route_spec_http_2_route_match_header_match_range.rs");
        include!("types/appmesh/route_spec_http_2_route_match_path.rs");
        include!("types/appmesh/route_spec_http_2_route_match_query_parameter.rs");
        include!("types/appmesh/route_spec_http_2_route_match_query_parameter_match.rs");
        include!("types/appmesh/route_spec_http_2_route_retry_policy.rs");
        include!(
            "types/appmesh/route_spec_http_2_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/route_spec_http_2_route_timeout.rs");
        include!("types/appmesh/route_spec_http_2_route_timeout_idle.rs");
        include!("types/appmesh/route_spec_http_2_route_timeout_per_request.rs");
        include!("types/appmesh/route_spec_http_route.rs");
        include!("types/appmesh/route_spec_http_route_action.rs");
        include!("types/appmesh/route_spec_http_route_action_weighted_target.rs");
        include!("types/appmesh/route_spec_http_route_match.rs");
        include!("types/appmesh/route_spec_http_route_match_header.rs");
        include!("types/appmesh/route_spec_http_route_match_header_match.rs");
        include!("types/appmesh/route_spec_http_route_match_header_match_range.rs");
        include!("types/appmesh/route_spec_http_route_match_path.rs");
        include!("types/appmesh/route_spec_http_route_match_query_parameter.rs");
        include!("types/appmesh/route_spec_http_route_match_query_parameter_match.rs");
        include!("types/appmesh/route_spec_http_route_retry_policy.rs");
        include!(
            "types/appmesh/route_spec_http_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/route_spec_http_route_timeout.rs");
        include!("types/appmesh/route_spec_http_route_timeout_idle.rs");
        include!("types/appmesh/route_spec_http_route_timeout_per_request.rs");
        include!("types/appmesh/route_spec_tcp_route.rs");
        include!("types/appmesh/route_spec_tcp_route_action.rs");
        include!("types/appmesh/route_spec_tcp_route_action_weighted_target.rs");
        include!("types/appmesh/route_spec_tcp_route_match.rs");
        include!("types/appmesh/route_spec_tcp_route_timeout.rs");
        include!("types/appmesh/route_spec_tcp_route_timeout_idle.rs");
        include!("types/appmesh/virtual_gateway_spec.rs");
        include!("types/appmesh/virtual_gateway_spec_backend_defaults.rs");
        include!("types/appmesh/virtual_gateway_spec_backend_defaults_client_policy.rs");
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_certificate.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_certificate_file.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_certificate_sds.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_subject_alternative_names.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_subject_alternative_names_match.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_trust.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_backend_defaults_client_policy_tls_validation_trust_sds.rs"
        );
        include!("types/appmesh/virtual_gateway_spec_listener.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_connection_pool.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_connection_pool_grpc.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_connection_pool_http.rs");
        include!(
            "types/appmesh/virtual_gateway_spec_listener_connection_pool_http_2.rs"
        );
        include!("types/appmesh/virtual_gateway_spec_listener_health_check.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_port_mapping.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls_certificate.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls_certificate_acm.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls_certificate_file.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls_certificate_sds.rs");
        include!("types/appmesh/virtual_gateway_spec_listener_tls_validation.rs");
        include!(
            "types/appmesh/virtual_gateway_spec_listener_tls_validation_subject_alternative_names.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_listener_tls_validation_subject_alternative_names_match.rs"
        );
        include!("types/appmesh/virtual_gateway_spec_listener_tls_validation_trust.rs");
        include!(
            "types/appmesh/virtual_gateway_spec_listener_tls_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/virtual_gateway_spec_listener_tls_validation_trust_sds.rs"
        );
        include!("types/appmesh/virtual_gateway_spec_logging.rs");
        include!("types/appmesh/virtual_gateway_spec_logging_access_log.rs");
        include!("types/appmesh/virtual_gateway_spec_logging_access_log_file.rs");
        include!("types/appmesh/virtual_gateway_spec_logging_access_log_file_format.rs");
        include!(
            "types/appmesh/virtual_gateway_spec_logging_access_log_file_format_json.rs"
        );
        include!("types/appmesh/virtual_node_spec.rs");
        include!("types/appmesh/virtual_node_spec_backend.rs");
        include!("types/appmesh/virtual_node_spec_backend_defaults.rs");
        include!("types/appmesh/virtual_node_spec_backend_defaults_client_policy.rs");
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_certificate.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_certificate_file.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_certificate_sds.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_subject_alternative_names.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_subject_alternative_names_match.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_trust.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_defaults_client_policy_tls_validation_trust_sds.rs"
        );
        include!("types/appmesh/virtual_node_spec_backend_virtual_service.rs");
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_certificate.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_certificate_file.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_certificate_sds.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_subject_alternative_names.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_subject_alternative_names_match.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_trust.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_backend_virtual_service_client_policy_tls_validation_trust_sds.rs"
        );
        include!("types/appmesh/virtual_node_spec_listener.rs");
        include!("types/appmesh/virtual_node_spec_listener_connection_pool.rs");
        include!("types/appmesh/virtual_node_spec_listener_connection_pool_grpc.rs");
        include!("types/appmesh/virtual_node_spec_listener_connection_pool_http.rs");
        include!("types/appmesh/virtual_node_spec_listener_connection_pool_http_2.rs");
        include!("types/appmesh/virtual_node_spec_listener_connection_pool_tcp.rs");
        include!("types/appmesh/virtual_node_spec_listener_health_check.rs");
        include!("types/appmesh/virtual_node_spec_listener_outlier_detection.rs");
        include!(
            "types/appmesh/virtual_node_spec_listener_outlier_detection_base_ejection_duration.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_listener_outlier_detection_interval.rs"
        );
        include!("types/appmesh/virtual_node_spec_listener_port_mapping.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_grpc.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_grpc_idle.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_grpc_per_request.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_http.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_http_2.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_http_2_idle.rs");
        include!(
            "types/appmesh/virtual_node_spec_listener_timeout_http_2_per_request.rs"
        );
        include!("types/appmesh/virtual_node_spec_listener_timeout_http_idle.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_http_per_request.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_tcp.rs");
        include!("types/appmesh/virtual_node_spec_listener_timeout_tcp_idle.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls_certificate.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls_certificate_acm.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls_certificate_file.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls_certificate_sds.rs");
        include!("types/appmesh/virtual_node_spec_listener_tls_validation.rs");
        include!(
            "types/appmesh/virtual_node_spec_listener_tls_validation_subject_alternative_names.rs"
        );
        include!(
            "types/appmesh/virtual_node_spec_listener_tls_validation_subject_alternative_names_match.rs"
        );
        include!("types/appmesh/virtual_node_spec_listener_tls_validation_trust.rs");
        include!(
            "types/appmesh/virtual_node_spec_listener_tls_validation_trust_file.rs"
        );
        include!("types/appmesh/virtual_node_spec_listener_tls_validation_trust_sds.rs");
        include!("types/appmesh/virtual_node_spec_logging.rs");
        include!("types/appmesh/virtual_node_spec_logging_access_log.rs");
        include!("types/appmesh/virtual_node_spec_logging_access_log_file.rs");
        include!("types/appmesh/virtual_node_spec_logging_access_log_file_format.rs");
        include!(
            "types/appmesh/virtual_node_spec_logging_access_log_file_format_json.rs"
        );
        include!("types/appmesh/virtual_node_spec_service_discovery.rs");
        include!("types/appmesh/virtual_node_spec_service_discovery_aws_cloud_map.rs");
        include!("types/appmesh/virtual_node_spec_service_discovery_dns.rs");
        include!("types/appmesh/virtual_router_spec.rs");
        include!("types/appmesh/virtual_router_spec_listener.rs");
        include!("types/appmesh/virtual_router_spec_listener_port_mapping.rs");
        include!("types/appmesh/virtual_service_spec.rs");
        include!("types/appmesh/virtual_service_spec_provider.rs");
        include!("types/appmesh/virtual_service_spec_provider_virtual_node.rs");
        include!("types/appmesh/virtual_service_spec_provider_virtual_router.rs");
        include!("types/appmesh/get_gateway_route_spec.rs");
        include!("types/appmesh/get_gateway_route_spec_grpc_route.rs");
        include!("types/appmesh/get_gateway_route_spec_grpc_route_action.rs");
        include!("types/appmesh/get_gateway_route_spec_grpc_route_action_target.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_grpc_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_grpc_route_match.rs");
        include!("types/appmesh/get_gateway_route_spec_http_2_route.rs");
        include!("types/appmesh/get_gateway_route_spec_http_2_route_action.rs");
        include!("types/appmesh/get_gateway_route_spec_http_2_route_action_rewrite.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_action_rewrite_hostname.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_action_rewrite_path.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_action_rewrite_prefix.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_2_route_action_target.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_2_route_match.rs");
        include!("types/appmesh/get_gateway_route_spec_http_2_route_match_header.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_match_header_match.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_match_header_match_range.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_2_route_match_hostname.rs");
        include!("types/appmesh/get_gateway_route_spec_http_2_route_match_path.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_match_query_parameter.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_2_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_route.rs");
        include!("types/appmesh/get_gateway_route_spec_http_route_action.rs");
        include!("types/appmesh/get_gateway_route_spec_http_route_action_rewrite.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_action_rewrite_hostname.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_action_rewrite_path.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_action_rewrite_prefix.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_route_action_target.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_action_target_virtual_service.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_route_match.rs");
        include!("types/appmesh/get_gateway_route_spec_http_route_match_header.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_match_header_match.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_match_header_match_range.rs"
        );
        include!("types/appmesh/get_gateway_route_spec_http_route_match_hostname.rs");
        include!("types/appmesh/get_gateway_route_spec_http_route_match_path.rs");
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_match_query_parameter.rs"
        );
        include!(
            "types/appmesh/get_gateway_route_spec_http_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/get_mesh_spec.rs");
        include!("types/appmesh/get_mesh_spec_egress_filter.rs");
        include!("types/appmesh/get_mesh_spec_service_discovery.rs");
        include!("types/appmesh/get_route_spec.rs");
        include!("types/appmesh/get_route_spec_grpc_route.rs");
        include!("types/appmesh/get_route_spec_grpc_route_action.rs");
        include!("types/appmesh/get_route_spec_grpc_route_action_weighted_target.rs");
        include!("types/appmesh/get_route_spec_grpc_route_match.rs");
        include!("types/appmesh/get_route_spec_grpc_route_match_metadata.rs");
        include!("types/appmesh/get_route_spec_grpc_route_match_metadata_match.rs");
        include!(
            "types/appmesh/get_route_spec_grpc_route_match_metadata_match_range.rs"
        );
        include!("types/appmesh/get_route_spec_grpc_route_retry_policy.rs");
        include!(
            "types/appmesh/get_route_spec_grpc_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/get_route_spec_grpc_route_timeout.rs");
        include!("types/appmesh/get_route_spec_grpc_route_timeout_idle.rs");
        include!("types/appmesh/get_route_spec_grpc_route_timeout_per_request.rs");
        include!("types/appmesh/get_route_spec_http_2_route.rs");
        include!("types/appmesh/get_route_spec_http_2_route_action.rs");
        include!("types/appmesh/get_route_spec_http_2_route_action_weighted_target.rs");
        include!("types/appmesh/get_route_spec_http_2_route_match.rs");
        include!("types/appmesh/get_route_spec_http_2_route_match_header.rs");
        include!("types/appmesh/get_route_spec_http_2_route_match_header_match.rs");
        include!(
            "types/appmesh/get_route_spec_http_2_route_match_header_match_range.rs"
        );
        include!("types/appmesh/get_route_spec_http_2_route_match_path.rs");
        include!("types/appmesh/get_route_spec_http_2_route_match_query_parameter.rs");
        include!(
            "types/appmesh/get_route_spec_http_2_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/get_route_spec_http_2_route_retry_policy.rs");
        include!(
            "types/appmesh/get_route_spec_http_2_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/get_route_spec_http_2_route_timeout.rs");
        include!("types/appmesh/get_route_spec_http_2_route_timeout_idle.rs");
        include!("types/appmesh/get_route_spec_http_2_route_timeout_per_request.rs");
        include!("types/appmesh/get_route_spec_http_route.rs");
        include!("types/appmesh/get_route_spec_http_route_action.rs");
        include!("types/appmesh/get_route_spec_http_route_action_weighted_target.rs");
        include!("types/appmesh/get_route_spec_http_route_match.rs");
        include!("types/appmesh/get_route_spec_http_route_match_header.rs");
        include!("types/appmesh/get_route_spec_http_route_match_header_match.rs");
        include!("types/appmesh/get_route_spec_http_route_match_header_match_range.rs");
        include!("types/appmesh/get_route_spec_http_route_match_path.rs");
        include!("types/appmesh/get_route_spec_http_route_match_query_parameter.rs");
        include!(
            "types/appmesh/get_route_spec_http_route_match_query_parameter_match.rs"
        );
        include!("types/appmesh/get_route_spec_http_route_retry_policy.rs");
        include!(
            "types/appmesh/get_route_spec_http_route_retry_policy_per_retry_timeout.rs"
        );
        include!("types/appmesh/get_route_spec_http_route_timeout.rs");
        include!("types/appmesh/get_route_spec_http_route_timeout_idle.rs");
        include!("types/appmesh/get_route_spec_http_route_timeout_per_request.rs");
        include!("types/appmesh/get_route_spec_tcp_route.rs");
        include!("types/appmesh/get_route_spec_tcp_route_action.rs");
        include!("types/appmesh/get_route_spec_tcp_route_action_weighted_target.rs");
        include!("types/appmesh/get_route_spec_tcp_route_match.rs");
        include!("types/appmesh/get_route_spec_tcp_route_timeout.rs");
        include!("types/appmesh/get_route_spec_tcp_route_timeout_idle.rs");
        include!("types/appmesh/get_virtual_gateway_spec.rs");
        include!("types/appmesh/get_virtual_gateway_spec_backend_default.rs");
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_certificate.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_certificate_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_certificate_sd.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_subject_alternative_name.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_subject_alternative_name_match.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_trust.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_backend_default_client_policy_tl_validation_trust_sd.rs"
        );
        include!("types/appmesh/get_virtual_gateway_spec_listener.rs");
        include!("types/appmesh/get_virtual_gateway_spec_listener_connection_pool.rs");
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_connection_pool_grpc.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_connection_pool_http.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_connection_pool_http_2.rs"
        );
        include!("types/appmesh/get_virtual_gateway_spec_listener_health_check.rs");
        include!("types/appmesh/get_virtual_gateway_spec_listener_port_mapping.rs");
        include!("types/appmesh/get_virtual_gateway_spec_listener_tl.rs");
        include!("types/appmesh/get_virtual_gateway_spec_listener_tl_certificate.rs");
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_certificate_acm.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_certificate_file.rs"
        );
        include!("types/appmesh/get_virtual_gateway_spec_listener_tl_certificate_sd.rs");
        include!("types/appmesh/get_virtual_gateway_spec_listener_tl_validation.rs");
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_validation_subject_alternative_name.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_validation_subject_alternative_name_match.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_validation_trust.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_listener_tl_validation_trust_sd.rs"
        );
        include!("types/appmesh/get_virtual_gateway_spec_logging.rs");
        include!("types/appmesh/get_virtual_gateway_spec_logging_access_log.rs");
        include!("types/appmesh/get_virtual_gateway_spec_logging_access_log_file.rs");
        include!(
            "types/appmesh/get_virtual_gateway_spec_logging_access_log_file_format.rs"
        );
        include!(
            "types/appmesh/get_virtual_gateway_spec_logging_access_log_file_format_json.rs"
        );
        include!("types/appmesh/get_virtual_node_spec.rs");
        include!("types/appmesh/get_virtual_node_spec_backend.rs");
        include!("types/appmesh/get_virtual_node_spec_backend_default.rs");
        include!("types/appmesh/get_virtual_node_spec_backend_default_client_policy.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_certificate.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_certificate_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_certificate_sd.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_subject_alternative_name.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_subject_alternative_name_match.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_trust.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_default_client_policy_tl_validation_trust_sd.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_backend_virtual_service.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_certificate.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_certificate_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_certificate_sd.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_subject_alternative_name.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_subject_alternative_name_match.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_trust.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_trust_acm.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_backend_virtual_service_client_policy_tl_validation_trust_sd.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_connection_pool.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_connection_pool_grpc.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_connection_pool_http.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_connection_pool_http_2.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_connection_pool_tcp.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_health_check.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_outlier_detection.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_outlier_detection_base_ejection_duration.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_listener_outlier_detection_interval.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_port_mapping.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_grpc.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_grpc_idle.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_timeout_grpc_per_request.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_http.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_http_2.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_http_2_idle.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_timeout_http_2_per_request.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_http_idle.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_timeout_http_per_request.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_tcp.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_timeout_tcp_idle.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl_certificate.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl_certificate_acm.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl_certificate_file.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl_certificate_sd.rs");
        include!("types/appmesh/get_virtual_node_spec_listener_tl_validation.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_tl_validation_subject_alternative_name.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_listener_tl_validation_subject_alternative_name_match.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_listener_tl_validation_trust.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_listener_tl_validation_trust_file.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_listener_tl_validation_trust_sd.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_logging.rs");
        include!("types/appmesh/get_virtual_node_spec_logging_access_log.rs");
        include!("types/appmesh/get_virtual_node_spec_logging_access_log_file.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_logging_access_log_file_format.rs"
        );
        include!(
            "types/appmesh/get_virtual_node_spec_logging_access_log_file_format_json.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_service_discovery.rs");
        include!(
            "types/appmesh/get_virtual_node_spec_service_discovery_aws_cloud_map.rs"
        );
        include!("types/appmesh/get_virtual_node_spec_service_discovery_dn.rs");
        include!("types/appmesh/get_virtual_router_spec.rs");
        include!("types/appmesh/get_virtual_router_spec_listener.rs");
        include!("types/appmesh/get_virtual_router_spec_listener_port_mapping.rs");
        include!("types/appmesh/get_virtual_service_spec.rs");
        include!("types/appmesh/get_virtual_service_spec_provider.rs");
        include!("types/appmesh/get_virtual_service_spec_provider_virtual_node.rs");
        include!("types/appmesh/get_virtual_service_spec_provider_virtual_router.rs");
    }
    pub mod apprunner {
        include!(
            "types/apprunner/custom_domain_association_certificate_validation_record.rs"
        );
        include!("types/apprunner/deployment_timeouts.rs");
        include!("types/apprunner/observability_configuration_trace_configuration.rs");
        include!("types/apprunner/service_encryption_configuration.rs");
        include!("types/apprunner/service_health_check_configuration.rs");
        include!("types/apprunner/service_instance_configuration.rs");
        include!("types/apprunner/service_network_configuration.rs");
        include!(
            "types/apprunner/service_network_configuration_egress_configuration.rs"
        );
        include!(
            "types/apprunner/service_network_configuration_ingress_configuration.rs"
        );
        include!("types/apprunner/service_observability_configuration.rs");
        include!("types/apprunner/service_source_configuration.rs");
        include!(
            "types/apprunner/service_source_configuration_authentication_configuration.rs"
        );
        include!("types/apprunner/service_source_configuration_code_repository.rs");
        include!(
            "types/apprunner/service_source_configuration_code_repository_code_configuration.rs"
        );
        include!(
            "types/apprunner/service_source_configuration_code_repository_code_configuration_code_configuration_values.rs"
        );
        include!(
            "types/apprunner/service_source_configuration_code_repository_source_code_version.rs"
        );
        include!("types/apprunner/service_source_configuration_image_repository.rs");
        include!(
            "types/apprunner/service_source_configuration_image_repository_image_configuration.rs"
        );
        include!("types/apprunner/vpc_ingress_connection_ingress_vpc_configuration.rs");
    }
    pub mod appstream {
        include!("types/appstream/directory_config_service_account_credentials.rs");
        include!("types/appstream/fleet_compute_capacity.rs");
        include!("types/appstream/fleet_domain_join_info.rs");
        include!("types/appstream/fleet_vpc_config.rs");
        include!("types/appstream/image_builder_access_endpoint.rs");
        include!("types/appstream/image_builder_domain_join_info.rs");
        include!("types/appstream/image_builder_vpc_config.rs");
        include!("types/appstream/stack_access_endpoint.rs");
        include!("types/appstream/stack_application_settings.rs");
        include!("types/appstream/stack_storage_connector.rs");
        include!("types/appstream/stack_streaming_experience_settings.rs");
        include!("types/appstream/stack_user_setting.rs");
        include!("types/appstream/get_image_application.rs");
        include!("types/appstream/get_image_application_icon_s_3_location.rs");
        include!("types/appstream/get_image_image_permission.rs");
        include!("types/appstream/get_image_state_change_reason.rs");
    }
    pub mod appsync {
        include!("types/appsync/data_source_dynamodb_config.rs");
        include!("types/appsync/data_source_dynamodb_config_delta_sync_config.rs");
        include!("types/appsync/data_source_elasticsearch_config.rs");
        include!("types/appsync/data_source_event_bridge_config.rs");
        include!("types/appsync/data_source_http_config.rs");
        include!("types/appsync/data_source_http_config_authorization_config.rs");
        include!(
            "types/appsync/data_source_http_config_authorization_config_aws_iam_config.rs"
        );
        include!("types/appsync/data_source_lambda_config.rs");
        include!("types/appsync/data_source_opensearchservice_config.rs");
        include!("types/appsync/data_source_relational_database_config.rs");
        include!(
            "types/appsync/data_source_relational_database_config_http_endpoint_config.rs"
        );
        include!("types/appsync/function_runtime.rs");
        include!("types/appsync/function_sync_config.rs");
        include!("types/appsync/function_sync_config_lambda_conflict_handler_config.rs");
        include!("types/appsync/graph_ql_api_additional_authentication_provider.rs");
        include!(
            "types/appsync/graph_ql_api_additional_authentication_provider_lambda_authorizer_config.rs"
        );
        include!(
            "types/appsync/graph_ql_api_additional_authentication_provider_openid_connect_config.rs"
        );
        include!(
            "types/appsync/graph_ql_api_additional_authentication_provider_user_pool_config.rs"
        );
        include!("types/appsync/graph_ql_api_enhanced_metrics_config.rs");
        include!("types/appsync/graph_ql_api_lambda_authorizer_config.rs");
        include!("types/appsync/graph_ql_api_log_config.rs");
        include!("types/appsync/graph_ql_api_openid_connect_config.rs");
        include!("types/appsync/graph_ql_api_user_pool_config.rs");
        include!("types/appsync/resolver_caching_config.rs");
        include!("types/appsync/resolver_pipeline_config.rs");
        include!("types/appsync/resolver_runtime.rs");
        include!("types/appsync/resolver_sync_config.rs");
        include!("types/appsync/resolver_sync_config_lambda_conflict_handler_config.rs");
        include!(
            "types/appsync/source_api_association_source_api_association_config.rs"
        );
        include!("types/appsync/source_api_association_timeouts.rs");
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
