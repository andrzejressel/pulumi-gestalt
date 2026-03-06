pub mod scheduler {
    include!("resources/scheduler/schedule.rs");
    include!("resources/scheduler/schedule_group.rs");
}
pub mod schemas {
    include!("resources/schemas/discoverer.rs");
    include!("resources/schemas/registry.rs");
    include!("resources/schemas/registry_policy.rs");
    include!("resources/schemas/schema.rs");
}
pub mod secretsmanager {
    include!("resources/secretsmanager/secret.rs");
    include!("resources/secretsmanager/secret_policy.rs");
    include!("resources/secretsmanager/secret_rotation.rs");
    include!("resources/secretsmanager/secret_version.rs");
}
pub mod securityhub {
    include!("resources/securityhub/account.rs");
    include!("resources/securityhub/action_target.rs");
    include!("resources/securityhub/automation_rule.rs");
    include!("resources/securityhub/configuration_policy.rs");
    include!("resources/securityhub/configuration_policy_association.rs");
    include!("resources/securityhub/finding_aggregator.rs");
    include!("resources/securityhub/insight.rs");
    include!("resources/securityhub/invite_accepter.rs");
    include!("resources/securityhub/member.rs");
    include!("resources/securityhub/organization_admin_account.rs");
    include!("resources/securityhub/organization_configuration.rs");
    include!("resources/securityhub/product_subscription.rs");
    include!("resources/securityhub/standards_control.rs");
    include!("resources/securityhub/standards_control_association.rs");
    include!("resources/securityhub/standards_subscription.rs");
}
pub mod securitylake {
    include!("resources/securitylake/aws_log_source.rs");
    include!("resources/securitylake/custom_log_source.rs");
    include!("resources/securitylake/data_lake.rs");
    include!("resources/securitylake/subscriber.rs");
    include!("resources/securitylake/subscriber_notification.rs");
}
pub mod serverlessrepository {
    include!("resources/serverlessrepository/cloud_formation_stack.rs");
}
pub mod servicecatalog {
    include!("resources/servicecatalog/appregistry_application.rs");
    include!("resources/servicecatalog/appregistry_attribute_group.rs");
    include!("resources/servicecatalog/appregistry_attribute_group_association.rs");
    include!("resources/servicecatalog/budget_resource_association.rs");
    include!("resources/servicecatalog/constraint.rs");
    include!("resources/servicecatalog/organizations_access.rs");
    include!("resources/servicecatalog/portfolio.rs");
    include!("resources/servicecatalog/portfolio_share.rs");
    include!("resources/servicecatalog/principal_portfolio_association.rs");
    include!("resources/servicecatalog/product.rs");
    include!("resources/servicecatalog/product_portfolio_association.rs");
    include!("resources/servicecatalog/provisioned_product.rs");
    include!("resources/servicecatalog/provisioning_artifact.rs");
    include!("resources/servicecatalog/service_action.rs");
    include!("resources/servicecatalog/tag_option.rs");
    include!("resources/servicecatalog/tag_option_resource_association.rs");
}
pub mod servicediscovery {
    include!("resources/servicediscovery/http_namespace.rs");
    include!("resources/servicediscovery/instance.rs");
    include!("resources/servicediscovery/private_dns_namespace.rs");
    include!("resources/servicediscovery/public_dns_namespace.rs");
    include!("resources/servicediscovery/service.rs");
}
pub mod servicequotas {
    include!("resources/servicequotas/service_quota.rs");
    include!("resources/servicequotas/template.rs");
    include!("resources/servicequotas/template_association.rs");
}
pub mod ses {
    include!("resources/ses/active_receipt_rule_set.rs");
    include!("resources/ses/configuration_set.rs");
    include!("resources/ses/domain_dkim.rs");
    include!("resources/ses/domain_identity.rs");
    include!("resources/ses/domain_identity_verification.rs");
    include!("resources/ses/email_identity.rs");
    include!("resources/ses/event_destination.rs");
    include!("resources/ses/identity_notification_topic.rs");
    include!("resources/ses/identity_policy.rs");
    include!("resources/ses/mail_from.rs");
    include!("resources/ses/receipt_filter.rs");
    include!("resources/ses/receipt_rule.rs");
    include!("resources/ses/receipt_rule_set.rs");
    include!("resources/ses/template.rs");
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
    pub mod secretsmanager {
        include!("functions/secretsmanager/get_random_password.rs");
        include!("functions/secretsmanager/get_secret.rs");
        include!("functions/secretsmanager/get_secret_rotation.rs");
        include!("functions/secretsmanager/get_secret_version.rs");
        include!("functions/secretsmanager/get_secret_versions.rs");
        include!("functions/secretsmanager/get_secrets.rs");
    }
    pub mod securityhub {
        include!("functions/securityhub/get_standards_control_associations.rs");
    }
    pub mod serverlessrepository {
        include!("functions/serverlessrepository/get_application.rs");
    }
    pub mod servicecatalog {
        include!("functions/servicecatalog/get_appregistry_application.rs");
        include!("functions/servicecatalog/get_appregistry_attribute_group.rs");
        include!(
            "functions/servicecatalog/get_appregistry_attribute_group_associations.rs"
        );
        include!("functions/servicecatalog/get_constraint.rs");
        include!("functions/servicecatalog/get_launch_paths.rs");
        include!("functions/servicecatalog/get_portfolio.rs");
        include!("functions/servicecatalog/get_portfolio_constraints.rs");
        include!("functions/servicecatalog/get_product.rs");
        include!("functions/servicecatalog/get_provisioning_artifacts.rs");
    }
    pub mod servicediscovery {
        include!("functions/servicediscovery/get_dns_namespace.rs");
        include!("functions/servicediscovery/get_http_namespace.rs");
        include!("functions/servicediscovery/get_service.rs");
    }
    pub mod servicequotas {
        include!("functions/servicequotas/get_service.rs");
        include!("functions/servicequotas/get_service_quota.rs");
        include!("functions/servicequotas/get_templates.rs");
    }
    pub mod ses {
        include!("functions/ses/get_active_receipt_rule_set.rs");
        include!("functions/ses/get_domain_identity.rs");
        include!("functions/ses/get_email_identity.rs");
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
    pub mod scheduler {
        include!("types/scheduler/schedule_flexible_time_window.rs");
        include!("types/scheduler/schedule_target.rs");
        include!("types/scheduler/schedule_target_dead_letter_config.rs");
        include!("types/scheduler/schedule_target_ecs_parameters.rs");
        include!(
            "types/scheduler/schedule_target_ecs_parameters_capacity_provider_strategy.rs"
        );
        include!(
            "types/scheduler/schedule_target_ecs_parameters_network_configuration.rs"
        );
        include!(
            "types/scheduler/schedule_target_ecs_parameters_placement_constraint.rs"
        );
        include!("types/scheduler/schedule_target_ecs_parameters_placement_strategy.rs");
        include!("types/scheduler/schedule_target_eventbridge_parameters.rs");
        include!("types/scheduler/schedule_target_kinesis_parameters.rs");
        include!("types/scheduler/schedule_target_retry_policy.rs");
        include!("types/scheduler/schedule_target_sagemaker_pipeline_parameters.rs");
        include!(
            "types/scheduler/schedule_target_sagemaker_pipeline_parameters_pipeline_parameter.rs"
        );
        include!("types/scheduler/schedule_target_sqs_parameters.rs");
    }
    pub mod secretsmanager {
        include!("types/secretsmanager/secret_replica.rs");
        include!("types/secretsmanager/secret_rotation_rotation_rules.rs");
        include!("types/secretsmanager/get_secret_rotation_rotation_rule.rs");
        include!("types/secretsmanager/get_secret_versions_version.rs");
        include!("types/secretsmanager/get_secrets_filter.rs");
    }
    pub mod securityhub {
        include!("types/securityhub/automation_rule_action.rs");
        include!("types/securityhub/automation_rule_action_finding_fields_update.rs");
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_note.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_related_finding.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_severity.rs"
        );
        include!(
            "types/securityhub/automation_rule_action_finding_fields_update_workflow.rs"
        );
        include!("types/securityhub/automation_rule_criteria.rs");
        include!("types/securityhub/automation_rule_criteria_aws_account_id.rs");
        include!("types/securityhub/automation_rule_criteria_aws_account_name.rs");
        include!("types/securityhub/automation_rule_criteria_company_name.rs");
        include!(
            "types/securityhub/automation_rule_criteria_compliance_associated_standards_id.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_compliance_security_control_id.rs"
        );
        include!("types/securityhub/automation_rule_criteria_compliance_status.rs");
        include!("types/securityhub/automation_rule_criteria_confidence.rs");
        include!("types/securityhub/automation_rule_criteria_created_at.rs");
        include!("types/securityhub/automation_rule_criteria_created_at_date_range.rs");
        include!("types/securityhub/automation_rule_criteria_criticality.rs");
        include!("types/securityhub/automation_rule_criteria_description.rs");
        include!("types/securityhub/automation_rule_criteria_first_observed_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_first_observed_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_generator_id.rs");
        include!("types/securityhub/automation_rule_criteria_id.rs");
        include!("types/securityhub/automation_rule_criteria_last_observed_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_last_observed_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_note_text.rs");
        include!("types/securityhub/automation_rule_criteria_note_updated_at.rs");
        include!(
            "types/securityhub/automation_rule_criteria_note_updated_at_date_range.rs"
        );
        include!("types/securityhub/automation_rule_criteria_note_updated_by.rs");
        include!("types/securityhub/automation_rule_criteria_product_arn.rs");
        include!("types/securityhub/automation_rule_criteria_product_name.rs");
        include!("types/securityhub/automation_rule_criteria_record_state.rs");
        include!("types/securityhub/automation_rule_criteria_related_findings_id.rs");
        include!(
            "types/securityhub/automation_rule_criteria_related_findings_product_arn.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_resource_application_arn.rs"
        );
        include!(
            "types/securityhub/automation_rule_criteria_resource_application_name.rs"
        );
        include!("types/securityhub/automation_rule_criteria_resource_details_other.rs");
        include!("types/securityhub/automation_rule_criteria_resource_id.rs");
        include!("types/securityhub/automation_rule_criteria_resource_partition.rs");
        include!("types/securityhub/automation_rule_criteria_resource_region.rs");
        include!("types/securityhub/automation_rule_criteria_resource_tag.rs");
        include!("types/securityhub/automation_rule_criteria_resource_type.rs");
        include!("types/securityhub/automation_rule_criteria_severity_label.rs");
        include!("types/securityhub/automation_rule_criteria_source_url.rs");
        include!("types/securityhub/automation_rule_criteria_title.rs");
        include!("types/securityhub/automation_rule_criteria_type.rs");
        include!("types/securityhub/automation_rule_criteria_updated_at.rs");
        include!("types/securityhub/automation_rule_criteria_updated_at_date_range.rs");
        include!("types/securityhub/automation_rule_criteria_user_defined_field.rs");
        include!("types/securityhub/automation_rule_criteria_verification_state.rs");
        include!("types/securityhub/automation_rule_criteria_workflow_status.rs");
        include!("types/securityhub/configuration_policy_configuration_policy.rs");
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_bool.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_double.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_enum.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_enum_list.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_int.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_int_list.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_string.rs"
        );
        include!(
            "types/securityhub/configuration_policy_configuration_policy_security_controls_configuration_security_control_custom_parameter_parameter_string_list.rs"
        );
        include!("types/securityhub/insight_filters.rs");
        include!("types/securityhub/insight_filters_aws_account_id.rs");
        include!("types/securityhub/insight_filters_company_name.rs");
        include!("types/securityhub/insight_filters_compliance_status.rs");
        include!("types/securityhub/insight_filters_confidence.rs");
        include!("types/securityhub/insight_filters_created_at.rs");
        include!("types/securityhub/insight_filters_created_at_date_range.rs");
        include!("types/securityhub/insight_filters_criticality.rs");
        include!("types/securityhub/insight_filters_description.rs");
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_confidence.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_criticality.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_related_findings_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_related_findings_product_arn.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_severity_label.rs"
        );
        include!(
            "types/securityhub/insight_filters_finding_provider_fields_severity_original.rs"
        );
        include!("types/securityhub/insight_filters_finding_provider_fields_type.rs");
        include!("types/securityhub/insight_filters_first_observed_at.rs");
        include!("types/securityhub/insight_filters_first_observed_at_date_range.rs");
        include!("types/securityhub/insight_filters_generator_id.rs");
        include!("types/securityhub/insight_filters_id.rs");
        include!("types/securityhub/insight_filters_keyword.rs");
        include!("types/securityhub/insight_filters_last_observed_at.rs");
        include!("types/securityhub/insight_filters_last_observed_at_date_range.rs");
        include!("types/securityhub/insight_filters_malware_name.rs");
        include!("types/securityhub/insight_filters_malware_path.rs");
        include!("types/securityhub/insight_filters_malware_state.rs");
        include!("types/securityhub/insight_filters_malware_type.rs");
        include!("types/securityhub/insight_filters_network_destination_domain.rs");
        include!("types/securityhub/insight_filters_network_destination_ipv_4.rs");
        include!("types/securityhub/insight_filters_network_destination_ipv_6.rs");
        include!("types/securityhub/insight_filters_network_destination_port.rs");
        include!("types/securityhub/insight_filters_network_direction.rs");
        include!("types/securityhub/insight_filters_network_protocol.rs");
        include!("types/securityhub/insight_filters_network_source_domain.rs");
        include!("types/securityhub/insight_filters_network_source_ipv_4.rs");
        include!("types/securityhub/insight_filters_network_source_ipv_6.rs");
        include!("types/securityhub/insight_filters_network_source_mac.rs");
        include!("types/securityhub/insight_filters_network_source_port.rs");
        include!("types/securityhub/insight_filters_note_text.rs");
        include!("types/securityhub/insight_filters_note_updated_at.rs");
        include!("types/securityhub/insight_filters_note_updated_at_date_range.rs");
        include!("types/securityhub/insight_filters_note_updated_by.rs");
        include!("types/securityhub/insight_filters_process_launched_at.rs");
        include!("types/securityhub/insight_filters_process_launched_at_date_range.rs");
        include!("types/securityhub/insight_filters_process_name.rs");
        include!("types/securityhub/insight_filters_process_parent_pid.rs");
        include!("types/securityhub/insight_filters_process_path.rs");
        include!("types/securityhub/insight_filters_process_pid.rs");
        include!("types/securityhub/insight_filters_process_terminated_at.rs");
        include!(
            "types/securityhub/insight_filters_process_terminated_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_product_arn.rs");
        include!("types/securityhub/insight_filters_product_field.rs");
        include!("types/securityhub/insight_filters_product_name.rs");
        include!("types/securityhub/insight_filters_recommendation_text.rs");
        include!("types/securityhub/insight_filters_record_state.rs");
        include!("types/securityhub/insight_filters_related_findings_id.rs");
        include!("types/securityhub/insight_filters_related_findings_product_arn.rs");
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_iam_instance_profile_arn.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_image_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_ipv_4_address.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_ipv_6_address.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_key_name.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_launched_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_launched_at_date_range.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_subnet_id.rs"
        );
        include!("types/securityhub/insight_filters_resource_aws_ec_2_instance_type.rs");
        include!(
            "types/securityhub/insight_filters_resource_aws_ec_2_instance_vpc_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_created_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_created_at_date_range.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_status.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_iam_access_key_user_name.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_s_3_bucket_owner_id.rs"
        );
        include!(
            "types/securityhub/insight_filters_resource_aws_s_3_bucket_owner_name.rs"
        );
        include!("types/securityhub/insight_filters_resource_container_image_id.rs");
        include!("types/securityhub/insight_filters_resource_container_image_name.rs");
        include!("types/securityhub/insight_filters_resource_container_launched_at.rs");
        include!(
            "types/securityhub/insight_filters_resource_container_launched_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_resource_container_name.rs");
        include!("types/securityhub/insight_filters_resource_details_other.rs");
        include!("types/securityhub/insight_filters_resource_id.rs");
        include!("types/securityhub/insight_filters_resource_partition.rs");
        include!("types/securityhub/insight_filters_resource_region.rs");
        include!("types/securityhub/insight_filters_resource_tag.rs");
        include!("types/securityhub/insight_filters_resource_type.rs");
        include!("types/securityhub/insight_filters_severity_label.rs");
        include!("types/securityhub/insight_filters_source_url.rs");
        include!("types/securityhub/insight_filters_threat_intel_indicator_category.rs");
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_last_observed_at.rs"
        );
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_last_observed_at_date_range.rs"
        );
        include!("types/securityhub/insight_filters_threat_intel_indicator_source.rs");
        include!(
            "types/securityhub/insight_filters_threat_intel_indicator_source_url.rs"
        );
        include!("types/securityhub/insight_filters_threat_intel_indicator_type.rs");
        include!("types/securityhub/insight_filters_threat_intel_indicator_value.rs");
        include!("types/securityhub/insight_filters_title.rs");
        include!("types/securityhub/insight_filters_type.rs");
        include!("types/securityhub/insight_filters_updated_at.rs");
        include!("types/securityhub/insight_filters_updated_at_date_range.rs");
        include!("types/securityhub/insight_filters_user_defined_value.rs");
        include!("types/securityhub/insight_filters_verification_state.rs");
        include!("types/securityhub/insight_filters_workflow_status.rs");
        include!(
            "types/securityhub/organization_configuration_organization_configuration.rs"
        );
        include!(
            "types/securityhub/get_standards_control_associations_standards_control_association.rs"
        );
    }
    pub mod securitylake {
        include!("types/securitylake/aws_log_source_source.rs");
        include!("types/securitylake/custom_log_source_attribute.rs");
        include!("types/securitylake/custom_log_source_configuration.rs");
        include!(
            "types/securitylake/custom_log_source_configuration_crawler_configuration.rs"
        );
        include!(
            "types/securitylake/custom_log_source_configuration_provider_identity.rs"
        );
        include!("types/securitylake/custom_log_source_provider_detail.rs");
        include!("types/securitylake/data_lake_configuration.rs");
        include!(
            "types/securitylake/data_lake_configuration_encryption_configuration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration_expiration.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_lifecycle_configuration_transition.rs"
        );
        include!(
            "types/securitylake/data_lake_configuration_replication_configuration.rs"
        );
        include!("types/securitylake/data_lake_timeouts.rs");
        include!("types/securitylake/subscriber_notification_configuration.rs");
        include!(
            "types/securitylake/subscriber_notification_configuration_https_notification_configuration.rs"
        );
        include!(
            "types/securitylake/subscriber_notification_configuration_sqs_notification_configuration.rs"
        );
        include!("types/securitylake/subscriber_source.rs");
        include!("types/securitylake/subscriber_source_aws_log_source_resource.rs");
        include!("types/securitylake/subscriber_source_custom_log_source_resource.rs");
        include!(
            "types/securitylake/subscriber_source_custom_log_source_resource_attribute.rs"
        );
        include!(
            "types/securitylake/subscriber_source_custom_log_source_resource_provider.rs"
        );
        include!("types/securitylake/subscriber_subscriber_identity.rs");
        include!("types/securitylake/subscriber_timeouts.rs");
    }
    pub mod servicecatalog {
        include!("types/servicecatalog/product_provisioning_artifact_parameters.rs");
        include!("types/servicecatalog/provisioned_product_output.rs");
        include!("types/servicecatalog/provisioned_product_provisioning_parameter.rs");
        include!(
            "types/servicecatalog/provisioned_product_stack_set_provisioning_preferences.rs"
        );
        include!("types/servicecatalog/service_action_definition.rs");
        include!("types/servicecatalog/get_launch_paths_summary.rs");
        include!("types/servicecatalog/get_launch_paths_summary_constraint_summary.rs");
        include!("types/servicecatalog/get_portfolio_constraints_detail.rs");
        include!(
            "types/servicecatalog/get_provisioning_artifacts_provisioning_artifact_detail.rs"
        );
    }
    pub mod servicediscovery {
        include!("types/servicediscovery/service_dns_config.rs");
        include!("types/servicediscovery/service_dns_config_dns_record.rs");
        include!("types/servicediscovery/service_health_check_config.rs");
        include!("types/servicediscovery/service_health_check_custom_config.rs");
        include!("types/servicediscovery/get_service_dns_config.rs");
        include!("types/servicediscovery/get_service_dns_config_dns_record.rs");
        include!("types/servicediscovery/get_service_health_check_config.rs");
        include!("types/servicediscovery/get_service_health_check_custom_config.rs");
    }
    pub mod servicequotas {
        include!("types/servicequotas/service_quota_usage_metric.rs");
        include!("types/servicequotas/service_quota_usage_metric_metric_dimension.rs");
        include!("types/servicequotas/get_service_quota_usage_metric.rs");
        include!(
            "types/servicequotas/get_service_quota_usage_metric_metric_dimension.rs"
        );
        include!("types/servicequotas/get_templates_template.rs");
    }
    pub mod ses {
        include!("types/ses/configuration_set_delivery_options.rs");
        include!("types/ses/configuration_set_tracking_options.rs");
        include!("types/ses/event_destination_cloudwatch_destination.rs");
        include!("types/ses/event_destination_kinesis_destination.rs");
        include!("types/ses/event_destination_sns_destination.rs");
        include!("types/ses/receipt_rule_add_header_action.rs");
        include!("types/ses/receipt_rule_bounce_action.rs");
        include!("types/ses/receipt_rule_lambda_action.rs");
        include!("types/ses/receipt_rule_s_3_action.rs");
        include!("types/ses/receipt_rule_sns_action.rs");
        include!("types/ses/receipt_rule_stop_action.rs");
        include!("types/ses/receipt_rule_workmail_action.rs");
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
