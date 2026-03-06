pub mod accessanalyzer {
    include!("resources/accessanalyzer/analyzer.rs");
    include!("resources/accessanalyzer/archive_rule.rs");
}
pub mod account {
    include!("resources/account/alternative_contact.rs");
    include!("resources/account/primary_contact.rs");
    include!("resources/account/region.rs");
}
pub mod acm {
    include!("resources/acm/certificate.rs");
    include!("resources/acm/certificate_validation.rs");
}
pub mod acmpca {
    include!("resources/acmpca/certificate.rs");
    include!("resources/acmpca/certificate_authority.rs");
    include!("resources/acmpca/certificate_authority_certificate.rs");
    include!("resources/acmpca/permission.rs");
    include!("resources/acmpca/policy.rs");
}
pub mod alb {
    include!("resources/alb/listener.rs");
    include!("resources/alb/listener_certificate.rs");
    include!("resources/alb/listener_rule.rs");
    include!("resources/alb/load_balancer.rs");
    include!("resources/alb/target_group.rs");
    include!("resources/alb/target_group_attachment.rs");
}
pub mod amp {
    include!("resources/amp/alert_manager_definition.rs");
    include!("resources/amp/rule_group_namespace.rs");
    include!("resources/amp/scraper.rs");
    include!("resources/amp/workspace.rs");
}
pub mod amplify {
    include!("resources/amplify/app.rs");
    include!("resources/amplify/backend_environment.rs");
    include!("resources/amplify/branch.rs");
    include!("resources/amplify/domain_association.rs");
    include!("resources/amplify/webhook.rs");
}
pub mod apigateway {
    include!("resources/apigateway/account.rs");
    include!("resources/apigateway/api_key.rs");
    include!("resources/apigateway/authorizer.rs");
    include!("resources/apigateway/base_path_mapping.rs");
    include!("resources/apigateway/client_certificate.rs");
    include!("resources/apigateway/deployment.rs");
    include!("resources/apigateway/documentation_part.rs");
    include!("resources/apigateway/documentation_version.rs");
    include!("resources/apigateway/domain_name.rs");
    include!("resources/apigateway/domain_name_access_association.rs");
    include!("resources/apigateway/integration.rs");
    include!("resources/apigateway/integration_response.rs");
    include!("resources/apigateway/method.rs");
    include!("resources/apigateway/method_response.rs");
    include!("resources/apigateway/method_settings.rs");
    include!("resources/apigateway/model.rs");
    include!("resources/apigateway/request_validator.rs");
    include!("resources/apigateway/resource.rs");
    include!("resources/apigateway/response.rs");
    include!("resources/apigateway/rest_api.rs");
    include!("resources/apigateway/rest_api_policy.rs");
    include!("resources/apigateway/stage.rs");
    include!("resources/apigateway/usage_plan.rs");
    include!("resources/apigateway/usage_plan_key.rs");
    include!("resources/apigateway/vpc_link.rs");
}
pub mod apigatewayv2 {
    include!("resources/apigatewayv2/api.rs");
    include!("resources/apigatewayv2/api_mapping.rs");
    include!("resources/apigatewayv2/authorizer.rs");
    include!("resources/apigatewayv2/deployment.rs");
    include!("resources/apigatewayv2/domain_name.rs");
    include!("resources/apigatewayv2/integration.rs");
    include!("resources/apigatewayv2/integration_response.rs");
    include!("resources/apigatewayv2/model.rs");
    include!("resources/apigatewayv2/route.rs");
    include!("resources/apigatewayv2/route_response.rs");
    include!("resources/apigatewayv2/stage.rs");
    include!("resources/apigatewayv2/vpc_link.rs");
}
pub mod appautoscaling {
    include!("resources/appautoscaling/policy.rs");
    include!("resources/appautoscaling/scheduled_action.rs");
    include!("resources/appautoscaling/target.rs");
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
    pub mod acm {
        include!("functions/acm/get_certificate.rs");
    }
    pub mod acmpca {
        include!("functions/acmpca/get_certificate.rs");
        include!("functions/acmpca/get_certificate_authority.rs");
    }
    pub mod alb {
        include!("functions/alb/get_listener.rs");
        include!("functions/alb/get_load_balancer.rs");
        include!("functions/alb/get_target_group.rs");
    }
    pub mod amp {
        include!("functions/amp/get_default_scraper_configuration.rs");
        include!("functions/amp/get_workspace.rs");
        include!("functions/amp/get_workspaces.rs");
    }
    pub mod apigateway {
        include!("functions/apigateway/get_authorizer.rs");
        include!("functions/apigateway/get_authorizers.rs");
        include!("functions/apigateway/get_domain_name.rs");
        include!("functions/apigateway/get_export.rs");
        include!("functions/apigateway/get_key.rs");
        include!("functions/apigateway/get_resource.rs");
        include!("functions/apigateway/get_rest_api.rs");
        include!("functions/apigateway/get_sdk.rs");
        include!("functions/apigateway/get_vpc_link.rs");
    }
    pub mod apigatewayv2 {
        include!("functions/apigatewayv2/get_api.rs");
        include!("functions/apigatewayv2/get_apis.rs");
        include!("functions/apigatewayv2/get_export.rs");
        include!("functions/apigatewayv2/get_vpc_link.rs");
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
    pub mod accessanalyzer {
        include!("types/accessanalyzer/analyzer_configuration.rs");
        include!("types/accessanalyzer/analyzer_configuration_unused_access.rs");
        include!("types/accessanalyzer/archive_rule_filter.rs");
    }
    pub mod acm {
        include!("types/acm/certificate_domain_validation_option.rs");
        include!("types/acm/certificate_options.rs");
        include!("types/acm/certificate_renewal_summary.rs");
        include!("types/acm/certificate_validation_option.rs");
    }
    pub mod acmpca {
        include!(
            "types/acmpca/certificate_authority_certificate_authority_configuration.rs"
        );
        include!(
            "types/acmpca/certificate_authority_certificate_authority_configuration_subject.rs"
        );
        include!("types/acmpca/certificate_authority_revocation_configuration.rs");
        include!(
            "types/acmpca/certificate_authority_revocation_configuration_crl_configuration.rs"
        );
        include!(
            "types/acmpca/certificate_authority_revocation_configuration_ocsp_configuration.rs"
        );
        include!("types/acmpca/certificate_validity.rs");
        include!("types/acmpca/get_certificate_authority_revocation_configuration.rs");
        include!(
            "types/acmpca/get_certificate_authority_revocation_configuration_crl_configuration.rs"
        );
        include!(
            "types/acmpca/get_certificate_authority_revocation_configuration_ocsp_configuration.rs"
        );
    }
    pub mod alb {
        include!("types/alb/listener_default_action.rs");
        include!("types/alb/listener_default_action_authenticate_cognito.rs");
        include!("types/alb/listener_default_action_authenticate_oidc.rs");
        include!("types/alb/listener_default_action_fixed_response.rs");
        include!("types/alb/listener_default_action_forward.rs");
        include!("types/alb/listener_default_action_forward_stickiness.rs");
        include!("types/alb/listener_default_action_forward_target_group.rs");
        include!("types/alb/listener_default_action_redirect.rs");
        include!("types/alb/listener_mutual_authentication.rs");
        include!("types/alb/listener_rule_action.rs");
        include!("types/alb/listener_rule_action_authenticate_cognito.rs");
        include!("types/alb/listener_rule_action_authenticate_oidc.rs");
        include!("types/alb/listener_rule_action_fixed_response.rs");
        include!("types/alb/listener_rule_action_forward.rs");
        include!("types/alb/listener_rule_action_forward_stickiness.rs");
        include!("types/alb/listener_rule_action_forward_target_group.rs");
        include!("types/alb/listener_rule_action_redirect.rs");
        include!("types/alb/listener_rule_condition.rs");
        include!("types/alb/listener_rule_condition_host_header.rs");
        include!("types/alb/listener_rule_condition_http_header.rs");
        include!("types/alb/listener_rule_condition_http_request_method.rs");
        include!("types/alb/listener_rule_condition_path_pattern.rs");
        include!("types/alb/listener_rule_condition_query_string.rs");
        include!("types/alb/listener_rule_condition_source_ip.rs");
        include!("types/alb/load_balancer_access_logs.rs");
        include!("types/alb/load_balancer_connection_logs.rs");
        include!("types/alb/load_balancer_subnet_mapping.rs");
        include!("types/alb/target_group_health_check.rs");
        include!("types/alb/target_group_stickiness.rs");
        include!("types/alb/target_group_target_failover.rs");
        include!("types/alb/target_group_target_group_health.rs");
        include!("types/alb/target_group_target_group_health_dns_failover.rs");
        include!(
            "types/alb/target_group_target_group_health_unhealthy_state_routing.rs"
        );
        include!("types/alb/target_group_target_health_state.rs");
        include!("types/alb/get_listener_default_action.rs");
        include!("types/alb/get_listener_default_action_authenticate_cognito.rs");
        include!("types/alb/get_listener_default_action_authenticate_oidc.rs");
        include!("types/alb/get_listener_default_action_fixed_response.rs");
        include!("types/alb/get_listener_default_action_forward.rs");
        include!("types/alb/get_listener_default_action_forward_stickiness.rs");
        include!("types/alb/get_listener_default_action_forward_target_group.rs");
        include!("types/alb/get_listener_default_action_redirect.rs");
        include!("types/alb/get_listener_mutual_authentication.rs");
        include!("types/alb/get_load_balancer_access_logs.rs");
        include!("types/alb/get_load_balancer_connection_log.rs");
        include!("types/alb/get_load_balancer_subnet_mapping.rs");
        include!("types/alb/get_target_group_health_check.rs");
        include!("types/alb/get_target_group_stickiness.rs");
    }
    pub mod amp {
        include!("types/amp/scraper_destination.rs");
        include!("types/amp/scraper_destination_amp.rs");
        include!("types/amp/scraper_source.rs");
        include!("types/amp/scraper_source_eks.rs");
        include!("types/amp/scraper_timeouts.rs");
        include!("types/amp/workspace_logging_configuration.rs");
    }
    pub mod amplify {
        include!("types/amplify/app_auto_branch_creation_config.rs");
        include!("types/amplify/app_cache_config.rs");
        include!("types/amplify/app_custom_rule.rs");
        include!("types/amplify/app_production_branch.rs");
        include!("types/amplify/domain_association_certificate_settings.rs");
        include!("types/amplify/domain_association_sub_domain.rs");
    }
    pub mod apigateway {
        include!("types/apigateway/account_throttle_setting.rs");
        include!("types/apigateway/deployment_canary_settings.rs");
        include!("types/apigateway/documentation_part_location.rs");
        include!("types/apigateway/domain_name_endpoint_configuration.rs");
        include!("types/apigateway/domain_name_mutual_tls_authentication.rs");
        include!("types/apigateway/integration_tls_config.rs");
        include!("types/apigateway/method_settings_settings.rs");
        include!("types/apigateway/rest_api_endpoint_configuration.rs");
        include!("types/apigateway/stage_access_log_settings.rs");
        include!("types/apigateway/stage_canary_settings.rs");
        include!("types/apigateway/usage_plan_api_stage.rs");
        include!("types/apigateway/usage_plan_api_stage_throttle.rs");
        include!("types/apigateway/usage_plan_quota_settings.rs");
        include!("types/apigateway/usage_plan_throttle_settings.rs");
        include!("types/apigateway/get_domain_name_endpoint_configuration.rs");
        include!("types/apigateway/get_rest_api_endpoint_configuration.rs");
    }
    pub mod apigatewayv2 {
        include!("types/apigatewayv2/api_cors_configuration.rs");
        include!("types/apigatewayv2/authorizer_jwt_configuration.rs");
        include!("types/apigatewayv2/domain_name_domain_name_configuration.rs");
        include!("types/apigatewayv2/domain_name_mutual_tls_authentication.rs");
        include!("types/apigatewayv2/integration_response_parameter.rs");
        include!("types/apigatewayv2/integration_tls_config.rs");
        include!("types/apigatewayv2/route_request_parameter.rs");
        include!("types/apigatewayv2/stage_access_log_settings.rs");
        include!("types/apigatewayv2/stage_default_route_settings.rs");
        include!("types/apigatewayv2/stage_route_setting.rs");
        include!("types/apigatewayv2/get_api_cors_configuration.rs");
    }
    pub mod appautoscaling {
        include!("types/appautoscaling/policy_step_scaling_policy_configuration.rs");
        include!(
            "types/appautoscaling/policy_step_scaling_policy_configuration_step_adjustment.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_dimension.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat_metric.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_customized_metric_specification_metric_metric_stat_metric_dimension.rs"
        );
        include!(
            "types/appautoscaling/policy_target_tracking_scaling_policy_configuration_predefined_metric_specification.rs"
        );
        include!("types/appautoscaling/scheduled_action_scalable_target_action.rs");
        include!("types/appautoscaling/target_suspended_state.rs");
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
