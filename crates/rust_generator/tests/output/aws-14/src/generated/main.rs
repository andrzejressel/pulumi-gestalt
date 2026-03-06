pub mod mwaa {
    include!("resources/mwaa/environment.rs");
}
pub mod neptune {
    include!("resources/neptune/cluster.rs");
    include!("resources/neptune/cluster_endpoint.rs");
    include!("resources/neptune/cluster_instance.rs");
    include!("resources/neptune/cluster_parameter_group.rs");
    include!("resources/neptune/cluster_snapshot.rs");
    include!("resources/neptune/event_subscription.rs");
    include!("resources/neptune/global_cluster.rs");
    include!("resources/neptune/parameter_group.rs");
    include!("resources/neptune/subnet_group.rs");
}
pub mod networkfirewall {
    include!("resources/networkfirewall/firewall.rs");
    include!("resources/networkfirewall/firewall_policy.rs");
    include!("resources/networkfirewall/logging_configuration.rs");
    include!("resources/networkfirewall/resource_policy.rs");
    include!("resources/networkfirewall/rule_group.rs");
    include!("resources/networkfirewall/tls_inspection_configuration.rs");
}
pub mod networkmanager {
    include!("resources/networkmanager/attachment_accepter.rs");
    include!("resources/networkmanager/connect_attachment.rs");
    include!("resources/networkmanager/connect_peer.rs");
    include!("resources/networkmanager/connection.rs");
    include!("resources/networkmanager/core_network.rs");
    include!("resources/networkmanager/core_network_policy_attachment.rs");
    include!("resources/networkmanager/customer_gateway_association.rs");
    include!("resources/networkmanager/device.rs");
    include!("resources/networkmanager/dx_gateway_attachment.rs");
    include!("resources/networkmanager/global_network.rs");
    include!("resources/networkmanager/link.rs");
    include!("resources/networkmanager/link_association.rs");
    include!("resources/networkmanager/site.rs");
    include!("resources/networkmanager/site_to_site_vpn_attachment.rs");
    include!("resources/networkmanager/transit_gateway_connect_peer_association.rs");
    include!("resources/networkmanager/transit_gateway_peering.rs");
    include!("resources/networkmanager/transit_gateway_registration.rs");
    include!("resources/networkmanager/transit_gateway_route_table_attachment.rs");
    include!("resources/networkmanager/vpc_attachment.rs");
}
pub mod networkmonitor {
    include!("resources/networkmonitor/monitor.rs");
    include!("resources/networkmonitor/probe.rs");
}
pub mod oam {
    include!("resources/oam/link.rs");
    include!("resources/oam/sink.rs");
    include!("resources/oam/sink_policy.rs");
}
pub mod opensearch {
    include!("resources/opensearch/authorize_vpc_endpoint_access.rs");
    include!("resources/opensearch/domain.rs");
    include!("resources/opensearch/domain_policy.rs");
    include!("resources/opensearch/domain_saml_options.rs");
    include!("resources/opensearch/inbound_connection_accepter.rs");
    include!("resources/opensearch/outbound_connection.rs");
    include!("resources/opensearch/package.rs");
    include!("resources/opensearch/package_association.rs");
    include!("resources/opensearch/serverless_access_policy.rs");
    include!("resources/opensearch/serverless_collection.rs");
    include!("resources/opensearch/serverless_lifecycle_policy.rs");
    include!("resources/opensearch/serverless_security_config.rs");
    include!("resources/opensearch/serverless_security_policy.rs");
    include!("resources/opensearch/serverless_vpc_endpoint.rs");
    include!("resources/opensearch/vpc_endpoint.rs");
}
pub mod opensearchingest {
    include!("resources/opensearchingest/pipeline.rs");
}
pub mod opsworks {
    include!("resources/opsworks/application.rs");
    include!("resources/opsworks/custom_layer.rs");
    include!("resources/opsworks/ecs_cluster_layer.rs");
    include!("resources/opsworks/ganglia_layer.rs");
    include!("resources/opsworks/haproxy_layer.rs");
    include!("resources/opsworks/instance.rs");
    include!("resources/opsworks/java_app_layer.rs");
    include!("resources/opsworks/memcached_layer.rs");
    include!("resources/opsworks/mysql_layer.rs");
    include!("resources/opsworks/nodejs_app_layer.rs");
    include!("resources/opsworks/permission.rs");
    include!("resources/opsworks/php_app_layer.rs");
    include!("resources/opsworks/rails_app_layer.rs");
    include!("resources/opsworks/rds_db_instance.rs");
    include!("resources/opsworks/stack.rs");
    include!("resources/opsworks/static_web_layer.rs");
    include!("resources/opsworks/user_profile.rs");
}
pub mod organizations {
    include!("resources/organizations/account.rs");
    include!("resources/organizations/delegated_administrator.rs");
    include!("resources/organizations/organization.rs");
    include!("resources/organizations/organizational_unit.rs");
    include!("resources/organizations/policy.rs");
    include!("resources/organizations/policy_attachment.rs");
    include!("resources/organizations/resource_policy.rs");
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
    pub mod neptune {
        include!("functions/neptune/get_engine_version.rs");
        include!("functions/neptune/get_orderable_db_instance.rs");
    }
    pub mod networkfirewall {
        include!("functions/networkfirewall/get_firewall.rs");
        include!("functions/networkfirewall/get_firewall_policy.rs");
        include!("functions/networkfirewall/get_resource_policy.rs");
    }
    pub mod networkmanager {
        include!("functions/networkmanager/get_connection.rs");
        include!("functions/networkmanager/get_connections.rs");
        include!("functions/networkmanager/get_core_network_policy_document.rs");
        include!("functions/networkmanager/get_device.rs");
        include!("functions/networkmanager/get_devices.rs");
        include!("functions/networkmanager/get_global_network.rs");
        include!("functions/networkmanager/get_global_networks.rs");
        include!("functions/networkmanager/get_link.rs");
        include!("functions/networkmanager/get_links.rs");
        include!("functions/networkmanager/get_site.rs");
        include!("functions/networkmanager/get_sites.rs");
    }
    pub mod oam {
        include!("functions/oam/get_link.rs");
        include!("functions/oam/get_links.rs");
        include!("functions/oam/get_sink.rs");
        include!("functions/oam/get_sinks.rs");
    }
    pub mod opensearch {
        include!("functions/opensearch/get_domain.rs");
        include!("functions/opensearch/get_serverless_access_policy.rs");
        include!("functions/opensearch/get_serverless_collection.rs");
        include!("functions/opensearch/get_serverless_lifecycle_policy.rs");
        include!("functions/opensearch/get_serverless_security_config.rs");
        include!("functions/opensearch/get_serverless_security_policy.rs");
        include!("functions/opensearch/get_serverless_vpc_endpoint.rs");
    }
    pub mod organizations {
        include!("functions/organizations/get_delegated_administrators.rs");
        include!("functions/organizations/get_delegated_services.rs");
        include!("functions/organizations/get_organization.rs");
        include!("functions/organizations/get_organizational_unit.rs");
        include!("functions/organizations/get_organizational_unit_child_accounts.rs");
        include!(
            "functions/organizations/get_organizational_unit_descendant_accounts.rs"
        );
        include!(
            "functions/organizations/get_organizational_unit_descendant_organizational_units.rs"
        );
        include!("functions/organizations/get_organizational_units.rs");
        include!("functions/organizations/get_policies.rs");
        include!("functions/organizations/get_policies_for_target.rs");
        include!("functions/organizations/get_policy.rs");
        include!("functions/organizations/get_resource_tags.rs");
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
    pub mod mwaa {
        include!("types/mwaa/environment_last_updated.rs");
        include!("types/mwaa/environment_last_updated_error.rs");
        include!("types/mwaa/environment_logging_configuration.rs");
        include!("types/mwaa/environment_logging_configuration_dag_processing_logs.rs");
        include!("types/mwaa/environment_logging_configuration_scheduler_logs.rs");
        include!("types/mwaa/environment_logging_configuration_task_logs.rs");
        include!("types/mwaa/environment_logging_configuration_webserver_logs.rs");
        include!("types/mwaa/environment_logging_configuration_worker_logs.rs");
        include!("types/mwaa/environment_network_configuration.rs");
    }
    pub mod neptune {
        include!("types/neptune/cluster_parameter_group_parameter.rs");
        include!("types/neptune/cluster_serverless_v_2_scaling_configuration.rs");
        include!("types/neptune/global_cluster_global_cluster_member.rs");
        include!("types/neptune/parameter_group_parameter.rs");
    }
    pub mod networkfirewall {
        include!("types/networkfirewall/firewall_encryption_configuration.rs");
        include!("types/networkfirewall/firewall_firewall_status.rs");
        include!("types/networkfirewall/firewall_firewall_status_sync_state.rs");
        include!(
            "types/networkfirewall/firewall_firewall_status_sync_state_attachment.rs"
        );
        include!("types/networkfirewall/firewall_policy_encryption_configuration.rs");
        include!("types/networkfirewall/firewall_policy_firewall_policy.rs");
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables_rule_variable.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_policy_variables_rule_variable_ip_set.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_engine_options.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_engine_options_flow_timeouts.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_rule_group_reference.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateful_rule_group_reference_override.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/firewall_policy_firewall_policy_stateless_rule_group_reference.rs"
        );
        include!("types/networkfirewall/firewall_subnet_mapping.rs");
        include!("types/networkfirewall/logging_configuration_logging_configuration.rs");
        include!(
            "types/networkfirewall/logging_configuration_logging_configuration_log_destination_config.rs"
        );
        include!("types/networkfirewall/rule_group_encryption_configuration.rs");
        include!("types/networkfirewall/rule_group_rule_group.rs");
        include!("types/networkfirewall/rule_group_rule_group_reference_sets.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_reference_sets_ip_set_reference.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_reference_sets_ip_set_reference_ip_set_reference.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_rule_variables.rs");
        include!("types/networkfirewall/rule_group_rule_group_rule_variables_ip_set.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_ip_set_ip_set.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_port_set.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rule_variables_port_set_port_set.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_rules_source.rs");
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_rules_source_list.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule_header.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateful_rule_rule_option.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_destination.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_destination_port.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_source.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_source_port.rs"
        );
        include!(
            "types/networkfirewall/rule_group_rule_group_rules_source_stateless_rules_and_custom_actions_stateless_rule_rule_definition_match_attributes_tcp_flag.rs"
        );
        include!("types/networkfirewall/rule_group_rule_group_stateful_rule_options.rs");
        include!("types/networkfirewall/tls_inspection_configuration_certificate.rs");
        include!(
            "types/networkfirewall/tls_inspection_configuration_certificate_authority.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_encryption_configuration.rs"
        );
        include!("types/networkfirewall/tls_inspection_configuration_timeouts.rs");
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_check_certificate_revocation_status.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_destination.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_destination_port.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_source.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_scope_source_port.rs"
        );
        include!(
            "types/networkfirewall/tls_inspection_configuration_tls_inspection_configuration_server_certificate_configuration_server_certificate.rs"
        );
        include!("types/networkfirewall/get_firewall_encryption_configuration.rs");
        include!("types/networkfirewall/get_firewall_firewall_status.rs");
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary_cidr.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_firewall_status_capacity_usage_summary_cidr_ip_set_reference.rs"
        );
        include!("types/networkfirewall/get_firewall_firewall_status_sync_state.rs");
        include!(
            "types/networkfirewall/get_firewall_firewall_status_sync_state_attachment.rs"
        );
        include!("types/networkfirewall/get_firewall_policy_firewall_policy.rs");
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_engine_option.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_rule_group_reference.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateful_rule_group_reference_override.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_custom_action_action_definition_publish_metric_action_dimension.rs"
        );
        include!(
            "types/networkfirewall/get_firewall_policy_firewall_policy_stateless_rule_group_reference.rs"
        );
        include!("types/networkfirewall/get_firewall_subnet_mapping.rs");
    }
    pub mod networkmanager {
        include!("types/networkmanager/connect_attachment_options.rs");
        include!("types/networkmanager/connect_peer_bgp_options.rs");
        include!("types/networkmanager/connect_peer_configuration.rs");
        include!("types/networkmanager/connect_peer_configuration_bgp_configuration.rs");
        include!("types/networkmanager/core_network_edge.rs");
        include!("types/networkmanager/core_network_segment.rs");
        include!("types/networkmanager/device_aws_location.rs");
        include!("types/networkmanager/device_location.rs");
        include!("types/networkmanager/dx_gateway_attachment_timeouts.rs");
        include!("types/networkmanager/link_bandwidth.rs");
        include!("types/networkmanager/site_location.rs");
        include!("types/networkmanager/vpc_attachment_options.rs");
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy_action.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_attachment_policy_condition.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_core_network_configuration.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_core_network_configuration_edge_location.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_network_function_group.rs"
        );
        include!("types/networkmanager/get_core_network_policy_document_segment.rs");
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_via.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_via_with_edge_override.rs"
        );
        include!(
            "types/networkmanager/get_core_network_policy_document_segment_action_when_sent_to.rs"
        );
        include!("types/networkmanager/get_device_aws_location.rs");
        include!("types/networkmanager/get_device_location.rs");
        include!("types/networkmanager/get_link_bandwidth.rs");
        include!("types/networkmanager/get_site_location.rs");
    }
    pub mod oam {
        include!("types/oam/link_link_configuration.rs");
        include!("types/oam/link_link_configuration_log_group_configuration.rs");
        include!("types/oam/link_link_configuration_metric_configuration.rs");
        include!("types/oam/get_link_link_configuration.rs");
        include!("types/oam/get_link_link_configuration_log_group_configuration.rs");
        include!("types/oam/get_link_link_configuration_metric_configuration.rs");
    }
    pub mod opensearch {
        include!(
            "types/opensearch/authorize_vpc_endpoint_access_authorized_principal.rs"
        );
        include!("types/opensearch/domain_advanced_security_options.rs");
        include!(
            "types/opensearch/domain_advanced_security_options_master_user_options.rs"
        );
        include!("types/opensearch/domain_auto_tune_options.rs");
        include!("types/opensearch/domain_auto_tune_options_maintenance_schedule.rs");
        include!(
            "types/opensearch/domain_auto_tune_options_maintenance_schedule_duration.rs"
        );
        include!("types/opensearch/domain_cluster_config.rs");
        include!("types/opensearch/domain_cluster_config_cold_storage_options.rs");
        include!("types/opensearch/domain_cluster_config_zone_awareness_config.rs");
        include!("types/opensearch/domain_cognito_options.rs");
        include!("types/opensearch/domain_domain_endpoint_options.rs");
        include!("types/opensearch/domain_ebs_options.rs");
        include!("types/opensearch/domain_encrypt_at_rest.rs");
        include!("types/opensearch/domain_log_publishing_option.rs");
        include!("types/opensearch/domain_node_to_node_encryption.rs");
        include!("types/opensearch/domain_off_peak_window_options.rs");
        include!("types/opensearch/domain_off_peak_window_options_off_peak_window.rs");
        include!(
            "types/opensearch/domain_off_peak_window_options_off_peak_window_window_start_time.rs"
        );
        include!("types/opensearch/domain_saml_options_saml_options.rs");
        include!("types/opensearch/domain_saml_options_saml_options_idp.rs");
        include!("types/opensearch/domain_snapshot_options.rs");
        include!("types/opensearch/domain_software_update_options.rs");
        include!("types/opensearch/domain_vpc_options.rs");
        include!("types/opensearch/outbound_connection_connection_properties.rs");
        include!(
            "types/opensearch/outbound_connection_connection_properties_cross_cluster_search.rs"
        );
        include!("types/opensearch/outbound_connection_local_domain_info.rs");
        include!("types/opensearch/outbound_connection_remote_domain_info.rs");
        include!("types/opensearch/package_package_source.rs");
        include!("types/opensearch/serverless_collection_timeouts.rs");
        include!("types/opensearch/serverless_security_config_saml_options.rs");
        include!("types/opensearch/serverless_vpc_endpoint_timeouts.rs");
        include!("types/opensearch/vpc_endpoint_vpc_options.rs");
        include!("types/opensearch/get_domain_advanced_security_option.rs");
        include!("types/opensearch/get_domain_auto_tune_option.rs");
        include!("types/opensearch/get_domain_auto_tune_option_maintenance_schedule.rs");
        include!(
            "types/opensearch/get_domain_auto_tune_option_maintenance_schedule_duration.rs"
        );
        include!("types/opensearch/get_domain_cluster_config.rs");
        include!("types/opensearch/get_domain_cluster_config_cold_storage_option.rs");
        include!("types/opensearch/get_domain_cluster_config_zone_awareness_config.rs");
        include!("types/opensearch/get_domain_cognito_option.rs");
        include!("types/opensearch/get_domain_ebs_option.rs");
        include!("types/opensearch/get_domain_encryption_at_rest.rs");
        include!("types/opensearch/get_domain_log_publishing_option.rs");
        include!("types/opensearch/get_domain_node_to_node_encryption.rs");
        include!("types/opensearch/get_domain_off_peak_window_options.rs");
        include!(
            "types/opensearch/get_domain_off_peak_window_options_off_peak_window.rs"
        );
        include!(
            "types/opensearch/get_domain_off_peak_window_options_off_peak_window_window_start_time.rs"
        );
        include!("types/opensearch/get_domain_snapshot_option.rs");
        include!("types/opensearch/get_domain_software_update_option.rs");
        include!("types/opensearch/get_domain_vpc_option.rs");
        include!("types/opensearch/get_serverless_security_config_saml_options.rs");
    }
    pub mod opensearchingest {
        include!("types/opensearchingest/pipeline_buffer_options.rs");
        include!("types/opensearchingest/pipeline_encryption_at_rest_options.rs");
        include!("types/opensearchingest/pipeline_log_publishing_options.rs");
        include!(
            "types/opensearchingest/pipeline_log_publishing_options_cloudwatch_log_destination.rs"
        );
        include!("types/opensearchingest/pipeline_timeouts.rs");
        include!("types/opensearchingest/pipeline_vpc_options.rs");
    }
    pub mod opsworks {
        include!("types/opsworks/application_app_source.rs");
        include!("types/opsworks/application_environment.rs");
        include!("types/opsworks/application_ssl_configuration.rs");
        include!("types/opsworks/custom_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/custom_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/custom_layer_ebs_volume.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/custom_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/ecs_cluster_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/ecs_cluster_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/ecs_cluster_layer_ebs_volume.rs");
        include!("types/opsworks/ecs_cluster_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/ecs_cluster_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!(
            "types/opsworks/ecs_cluster_layer_load_based_auto_scaling_upscaling.rs"
        );
        include!("types/opsworks/ganglia_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/ganglia_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/ganglia_layer_ebs_volume.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/ganglia_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/haproxy_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/haproxy_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/haproxy_layer_ebs_volume.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/haproxy_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/instance_ebs_block_device.rs");
        include!("types/opsworks/instance_ephemeral_block_device.rs");
        include!("types/opsworks/instance_root_block_device.rs");
        include!("types/opsworks/java_app_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/java_app_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/java_app_layer_ebs_volume.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/java_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/memcached_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/memcached_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/memcached_layer_ebs_volume.rs");
        include!("types/opsworks/memcached_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/memcached_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/memcached_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/mysql_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/mysql_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/mysql_layer_ebs_volume.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/mysql_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/nodejs_app_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/nodejs_app_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/nodejs_app_layer_ebs_volume.rs");
        include!("types/opsworks/nodejs_app_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/nodejs_app_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/nodejs_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/php_app_layer_cloudwatch_configuration.rs");
        include!("types/opsworks/php_app_layer_cloudwatch_configuration_log_stream.rs");
        include!("types/opsworks/php_app_layer_ebs_volume.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling_downscaling.rs");
        include!("types/opsworks/php_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/rails_app_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/rails_app_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/rails_app_layer_ebs_volume.rs");
        include!("types/opsworks/rails_app_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/rails_app_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/rails_app_layer_load_based_auto_scaling_upscaling.rs");
        include!("types/opsworks/stack_custom_cookbooks_source.rs");
        include!("types/opsworks/static_web_layer_cloudwatch_configuration.rs");
        include!(
            "types/opsworks/static_web_layer_cloudwatch_configuration_log_stream.rs"
        );
        include!("types/opsworks/static_web_layer_ebs_volume.rs");
        include!("types/opsworks/static_web_layer_load_based_auto_scaling.rs");
        include!(
            "types/opsworks/static_web_layer_load_based_auto_scaling_downscaling.rs"
        );
        include!("types/opsworks/static_web_layer_load_based_auto_scaling_upscaling.rs");
    }
    pub mod organizations {
        include!("types/organizations/organization_account.rs");
        include!("types/organizations/organization_non_master_account.rs");
        include!("types/organizations/organization_root.rs");
        include!("types/organizations/organization_root_policy_type.rs");
        include!("types/organizations/organizational_unit_account.rs");
        include!(
            "types/organizations/get_delegated_administrators_delegated_administrator.rs"
        );
        include!("types/organizations/get_delegated_services_delegated_service.rs");
        include!("types/organizations/get_organization_account.rs");
        include!("types/organizations/get_organization_non_master_account.rs");
        include!("types/organizations/get_organization_root.rs");
        include!("types/organizations/get_organization_root_policy_type.rs");
        include!(
            "types/organizations/get_organizational_unit_child_accounts_account.rs"
        );
        include!(
            "types/organizations/get_organizational_unit_descendant_accounts_account.rs"
        );
        include!(
            "types/organizations/get_organizational_unit_descendant_organizational_units_children.rs"
        );
        include!("types/organizations/get_organizational_units_child.rs");
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
