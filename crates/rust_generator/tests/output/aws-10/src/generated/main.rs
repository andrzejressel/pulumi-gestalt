pub mod fms {
    include!("resources/fms/admin_account.rs");
    include!("resources/fms/policy.rs");
    include!("resources/fms/resource_set.rs");
}
pub mod fsx {
    include!("resources/fsx/backup.rs");
    include!("resources/fsx/data_repository_association.rs");
    include!("resources/fsx/file_cache.rs");
    include!("resources/fsx/lustre_file_system.rs");
    include!("resources/fsx/ontap_file_system.rs");
    include!("resources/fsx/ontap_storage_virtual_machine.rs");
    include!("resources/fsx/ontap_volume.rs");
    include!("resources/fsx/open_zfs_file_system.rs");
    include!("resources/fsx/open_zfs_snapshot.rs");
    include!("resources/fsx/open_zfs_volume.rs");
    include!("resources/fsx/windows_file_system.rs");
}
pub mod gamelift {
    include!("resources/gamelift/alias.rs");
    include!("resources/gamelift/build.rs");
    include!("resources/gamelift/fleet.rs");
    include!("resources/gamelift/game_server_group.rs");
    include!("resources/gamelift/game_session_queue.rs");
    include!("resources/gamelift/matchmaking_configuration.rs");
    include!("resources/gamelift/matchmaking_rule_set.rs");
    include!("resources/gamelift/script.rs");
}
pub mod glacier {
    include!("resources/glacier/vault.rs");
    include!("resources/glacier/vault_lock.rs");
}
pub mod globalaccelerator {
    include!("resources/globalaccelerator/accelerator.rs");
    include!("resources/globalaccelerator/cross_account_attachment.rs");
    include!("resources/globalaccelerator/custom_routing_accelerator.rs");
    include!("resources/globalaccelerator/custom_routing_endpoint_group.rs");
    include!("resources/globalaccelerator/custom_routing_listener.rs");
    include!("resources/globalaccelerator/endpoint_group.rs");
    include!("resources/globalaccelerator/listener.rs");
}
pub mod glue {
    include!("resources/glue/catalog_database.rs");
    include!("resources/glue/catalog_table.rs");
    include!("resources/glue/catalog_table_optimizer.rs");
    include!("resources/glue/classifier.rs");
    include!("resources/glue/connection.rs");
    include!("resources/glue/crawler.rs");
    include!("resources/glue/data_catalog_encryption_settings.rs");
    include!("resources/glue/data_quality_ruleset.rs");
    include!("resources/glue/dev_endpoint.rs");
    include!("resources/glue/job.rs");
    include!("resources/glue/ml_transform.rs");
    include!("resources/glue/partition.rs");
    include!("resources/glue/partition_index.rs");
    include!("resources/glue/registry.rs");
    include!("resources/glue/resource_policy.rs");
    include!("resources/glue/schema.rs");
    include!("resources/glue/security_configuration.rs");
    include!("resources/glue/trigger.rs");
    include!("resources/glue/user_defined_function.rs");
    include!("resources/glue/workflow.rs");
}
pub mod grafana {
    include!("resources/grafana/license_association.rs");
    include!("resources/grafana/role_association.rs");
    include!("resources/grafana/workspace.rs");
    include!("resources/grafana/workspace_api_key.rs");
    include!("resources/grafana/workspace_saml_configuration.rs");
    include!("resources/grafana/workspace_service_account.rs");
    include!("resources/grafana/workspace_service_account_token.rs");
}
pub mod guardduty {
    include!("resources/guardduty/detector.rs");
    include!("resources/guardduty/detector_feature.rs");
    include!("resources/guardduty/filter.rs");
    include!("resources/guardduty/ip_set.rs");
    include!("resources/guardduty/invite_accepter.rs");
    include!("resources/guardduty/malware_protection_plan.rs");
    include!("resources/guardduty/member.rs");
    include!("resources/guardduty/organization_admin_account.rs");
    include!("resources/guardduty/organization_configuration.rs");
    include!("resources/guardduty/organization_configuration_feature.rs");
    include!("resources/guardduty/publishing_destination.rs");
    include!("resources/guardduty/threat_intel_set.rs");
}
pub mod iam {
    include!("resources/iam/access_key.rs");
    include!("resources/iam/account_alias.rs");
    include!("resources/iam/account_password_policy.rs");
    include!("resources/iam/group.rs");
    include!("resources/iam/group_membership.rs");
    include!("resources/iam/group_policies_exclusive.rs");
    include!("resources/iam/group_policy.rs");
    include!("resources/iam/group_policy_attachment.rs");
    include!("resources/iam/group_policy_attachments_exclusive.rs");
    include!("resources/iam/instance_profile.rs");
    include!("resources/iam/open_id_connect_provider.rs");
    include!("resources/iam/organizations_features.rs");
    include!("resources/iam/policy.rs");
    include!("resources/iam/policy_attachment.rs");
    include!("resources/iam/role.rs");
    include!("resources/iam/role_policies_exclusive.rs");
    include!("resources/iam/role_policy.rs");
    include!("resources/iam/role_policy_attachment.rs");
    include!("resources/iam/role_policy_attachments_exclusive.rs");
    include!("resources/iam/saml_provider.rs");
    include!("resources/iam/security_token_service_preferences.rs");
    include!("resources/iam/server_certificate.rs");
    include!("resources/iam/service_linked_role.rs");
    include!("resources/iam/service_specific_credential.rs");
    include!("resources/iam/signing_certificate.rs");
    include!("resources/iam/ssh_key.rs");
    include!("resources/iam/user.rs");
    include!("resources/iam/user_group_membership.rs");
    include!("resources/iam/user_login_profile.rs");
    include!("resources/iam/user_policies_exclusive.rs");
    include!("resources/iam/user_policy.rs");
    include!("resources/iam/user_policy_attachment.rs");
    include!("resources/iam/user_policy_attachments_exclusive.rs");
    include!("resources/iam/virtual_mfa_device.rs");
}
pub mod identitystore {
    include!("resources/identitystore/group.rs");
    include!("resources/identitystore/group_membership.rs");
    include!("resources/identitystore/user.rs");
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
    pub mod fsx {
        include!("functions/fsx/get_ontap_file_system.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machine.rs");
        include!("functions/fsx/get_ontap_storage_virtual_machines.rs");
        include!("functions/fsx/get_open_zfs_snapshot.rs");
        include!("functions/fsx/get_windows_file_system.rs");
    }
    pub mod globalaccelerator {
        include!("functions/globalaccelerator/get_accelerator.rs");
        include!("functions/globalaccelerator/get_custom_routing_accelerator.rs");
    }
    pub mod glue {
        include!("functions/glue/get_catalog_table.rs");
        include!("functions/glue/get_connection.rs");
        include!("functions/glue/get_data_catalog_encryption_settings.rs");
        include!("functions/glue/get_registry.rs");
        include!("functions/glue/get_script.rs");
    }
    pub mod grafana {
        include!("functions/grafana/get_workspace.rs");
    }
    pub mod guardduty {
        include!("functions/guardduty/get_detector.rs");
        include!("functions/guardduty/get_finding_ids.rs");
    }
    pub mod iam {
        include!("functions/iam/get_access_keys.rs");
        include!("functions/iam/get_account_alias.rs");
        include!("functions/iam/get_group.rs");
        include!("functions/iam/get_instance_profile.rs");
        include!("functions/iam/get_instance_profiles.rs");
        include!("functions/iam/get_open_id_connect_provider.rs");
        include!("functions/iam/get_policy.rs");
        include!("functions/iam/get_policy_document.rs");
        include!("functions/iam/get_principal_policy_simulation.rs");
        include!("functions/iam/get_role.rs");
        include!("functions/iam/get_roles.rs");
        include!("functions/iam/get_saml_provider.rs");
        include!("functions/iam/get_server_certificate.rs");
        include!("functions/iam/get_session_context.rs");
        include!("functions/iam/get_user.rs");
        include!("functions/iam/get_user_ssh_key.rs");
        include!("functions/iam/get_users.rs");
    }
    pub mod identitystore {
        include!("functions/identitystore/get_group.rs");
        include!("functions/identitystore/get_groups.rs");
        include!("functions/identitystore/get_user.rs");
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
    pub mod fms {
        include!("types/fms/policy_exclude_map.rs");
        include!("types/fms/policy_include_map.rs");
        include!("types/fms/policy_security_service_policy_data.rs");
        include!("types/fms/policy_security_service_policy_data_policy_option.rs");
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_network_firewall_policy.rs"
        );
        include!(
            "types/fms/policy_security_service_policy_data_policy_option_third_party_firewall_policy.rs"
        );
        include!("types/fms/resource_set_resource_set.rs");
        include!("types/fms/resource_set_timeouts.rs");
    }
    pub mod fsx {
        include!("types/fsx/data_repository_association_s_3.rs");
        include!("types/fsx/data_repository_association_s_3_auto_export_policy.rs");
        include!("types/fsx/data_repository_association_s_3_auto_import_policy.rs");
        include!("types/fsx/file_cache_data_repository_association.rs");
        include!("types/fsx/file_cache_data_repository_association_nf.rs");
        include!("types/fsx/file_cache_lustre_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_log_configuration.rs");
        include!("types/fsx/file_cache_lustre_configuration_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_log_configuration.rs");
        include!("types/fsx/lustre_file_system_metadata_configuration.rs");
        include!("types/fsx/lustre_file_system_root_squash_configuration.rs");
        include!("types/fsx/ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/ontap_file_system_endpoint.rs");
        include!("types/fsx/ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/ontap_volume_aggregate_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_autocommit_period.rs");
        include!("types/fsx/ontap_volume_snaplock_configuration_retention_period.rs");
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_default_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_maximum_retention.rs"
        );
        include!(
            "types/fsx/ontap_volume_snaplock_configuration_retention_period_minimum_retention.rs"
        );
        include!("types/fsx/ontap_volume_tiering_policy.rs");
        include!("types/fsx/open_zfs_file_system_disk_iops_configuration.rs");
        include!("types/fsx/open_zfs_file_system_root_volume_configuration.rs");
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_nfs_exports_client_configuration.rs"
        );
        include!(
            "types/fsx/open_zfs_file_system_root_volume_configuration_user_and_group_quota.rs"
        );
        include!("types/fsx/open_zfs_volume_nfs_exports.rs");
        include!("types/fsx/open_zfs_volume_nfs_exports_client_configuration.rs");
        include!("types/fsx/open_zfs_volume_origin_snapshot.rs");
        include!("types/fsx/open_zfs_volume_user_and_group_quota.rs");
        include!("types/fsx/windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/windows_file_system_disk_iops_configuration.rs");
        include!("types/fsx/windows_file_system_self_managed_active_directory.rs");
        include!("types/fsx/get_ontap_file_system_disk_iops_configuration.rs");
        include!("types/fsx/get_ontap_file_system_endpoint.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_intercluster.rs");
        include!("types/fsx/get_ontap_file_system_endpoint_management.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration.rs"
        );
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_active_directory_configuration_self_managed_active_directory_configuration.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_iscsi.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_management.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_nf.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_endpoint_smb.rs");
        include!("types/fsx/get_ontap_storage_virtual_machine_filter.rs");
        include!(
            "types/fsx/get_ontap_storage_virtual_machine_lifecycle_transition_reason.rs"
        );
        include!("types/fsx/get_ontap_storage_virtual_machines_filter.rs");
        include!("types/fsx/get_open_zfs_snapshot_filter.rs");
        include!("types/fsx/get_windows_file_system_audit_log_configuration.rs");
        include!("types/fsx/get_windows_file_system_disk_iops_configuration.rs");
    }
    pub mod gamelift {
        include!("types/gamelift/alias_routing_strategy.rs");
        include!("types/gamelift/build_storage_location.rs");
        include!("types/gamelift/fleet_certificate_configuration.rs");
        include!("types/gamelift/fleet_ec_2_inbound_permission.rs");
        include!("types/gamelift/fleet_resource_creation_limit_policy.rs");
        include!("types/gamelift/fleet_runtime_configuration.rs");
        include!("types/gamelift/fleet_runtime_configuration_server_process.rs");
        include!("types/gamelift/game_server_group_auto_scaling_policy.rs");
        include!(
            "types/gamelift/game_server_group_auto_scaling_policy_target_tracking_configuration.rs"
        );
        include!("types/gamelift/game_server_group_instance_definition.rs");
        include!("types/gamelift/game_server_group_launch_template.rs");
        include!("types/gamelift/game_session_queue_player_latency_policy.rs");
        include!("types/gamelift/matchmaking_configuration_game_property.rs");
        include!("types/gamelift/script_storage_location.rs");
    }
    pub mod glacier {
        include!("types/glacier/vault_notification.rs");
    }
    pub mod globalaccelerator {
        include!("types/globalaccelerator/accelerator_attributes.rs");
        include!("types/globalaccelerator/accelerator_ip_set.rs");
        include!("types/globalaccelerator/cross_account_attachment_resource.rs");
        include!("types/globalaccelerator/custom_routing_accelerator_attributes.rs");
        include!("types/globalaccelerator/custom_routing_accelerator_ip_set.rs");
        include!(
            "types/globalaccelerator/custom_routing_endpoint_group_destination_configuration.rs"
        );
        include!(
            "types/globalaccelerator/custom_routing_endpoint_group_endpoint_configuration.rs"
        );
        include!("types/globalaccelerator/custom_routing_listener_port_range.rs");
        include!("types/globalaccelerator/endpoint_group_endpoint_configuration.rs");
        include!("types/globalaccelerator/endpoint_group_port_override.rs");
        include!("types/globalaccelerator/listener_port_range.rs");
        include!("types/globalaccelerator/get_accelerator_attribute.rs");
        include!("types/globalaccelerator/get_accelerator_ip_set.rs");
        include!("types/globalaccelerator/get_custom_routing_accelerator_attribute.rs");
        include!("types/globalaccelerator/get_custom_routing_accelerator_ip_set.rs");
    }
    pub mod glue {
        include!("types/glue/catalog_database_create_table_default_permission.rs");
        include!(
            "types/glue/catalog_database_create_table_default_permission_principal.rs"
        );
        include!("types/glue/catalog_database_federated_database.rs");
        include!("types/glue/catalog_database_target_database.rs");
        include!("types/glue/catalog_table_open_table_format_input.rs");
        include!("types/glue/catalog_table_open_table_format_input_iceberg_input.rs");
        include!("types/glue/catalog_table_optimizer_configuration.rs");
        include!(
            "types/glue/catalog_table_optimizer_configuration_orphan_file_deletion_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_orphan_file_deletion_configuration_iceberg_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_retention_configuration.rs"
        );
        include!(
            "types/glue/catalog_table_optimizer_configuration_retention_configuration_iceberg_configuration.rs"
        );
        include!("types/glue/catalog_table_partition_index.rs");
        include!("types/glue/catalog_table_partition_key.rs");
        include!("types/glue/catalog_table_storage_descriptor.rs");
        include!("types/glue/catalog_table_storage_descriptor_column.rs");
        include!("types/glue/catalog_table_storage_descriptor_schema_reference.rs");
        include!(
            "types/glue/catalog_table_storage_descriptor_schema_reference_schema_id.rs"
        );
        include!("types/glue/catalog_table_storage_descriptor_ser_de_info.rs");
        include!("types/glue/catalog_table_storage_descriptor_skewed_info.rs");
        include!("types/glue/catalog_table_storage_descriptor_sort_column.rs");
        include!("types/glue/catalog_table_target_table.rs");
        include!("types/glue/classifier_csv_classifier.rs");
        include!("types/glue/classifier_grok_classifier.rs");
        include!("types/glue/classifier_json_classifier.rs");
        include!("types/glue/classifier_xml_classifier.rs");
        include!("types/glue/connection_physical_connection_requirements.rs");
        include!("types/glue/crawler_catalog_target.rs");
        include!("types/glue/crawler_delta_target.rs");
        include!("types/glue/crawler_dynamodb_target.rs");
        include!("types/glue/crawler_hudi_target.rs");
        include!("types/glue/crawler_iceberg_target.rs");
        include!("types/glue/crawler_jdbc_target.rs");
        include!("types/glue/crawler_lake_formation_configuration.rs");
        include!("types/glue/crawler_lineage_configuration.rs");
        include!("types/glue/crawler_mongodb_target.rs");
        include!("types/glue/crawler_recrawl_policy.rs");
        include!("types/glue/crawler_s_3_target.rs");
        include!("types/glue/crawler_schema_change_policy.rs");
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings.rs"
        );
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings_connection_password_encryption.rs"
        );
        include!(
            "types/glue/data_catalog_encryption_settings_data_catalog_encryption_settings_encryption_at_rest.rs"
        );
        include!("types/glue/data_quality_ruleset_target_table.rs");
        include!("types/glue/job_command.rs");
        include!("types/glue/job_execution_property.rs");
        include!("types/glue/job_notification_property.rs");
        include!("types/glue/ml_transform_input_record_table.rs");
        include!("types/glue/ml_transform_parameters.rs");
        include!("types/glue/ml_transform_parameters_find_matches_parameters.rs");
        include!("types/glue/ml_transform_schema.rs");
        include!("types/glue/partition_index_partition_index.rs");
        include!("types/glue/partition_storage_descriptor.rs");
        include!("types/glue/partition_storage_descriptor_column.rs");
        include!("types/glue/partition_storage_descriptor_ser_de_info.rs");
        include!("types/glue/partition_storage_descriptor_skewed_info.rs");
        include!("types/glue/partition_storage_descriptor_sort_column.rs");
        include!("types/glue/security_configuration_encryption_configuration.rs");
        include!(
            "types/glue/security_configuration_encryption_configuration_cloudwatch_encryption.rs"
        );
        include!(
            "types/glue/security_configuration_encryption_configuration_job_bookmarks_encryption.rs"
        );
        include!(
            "types/glue/security_configuration_encryption_configuration_s_3_encryption.rs"
        );
        include!("types/glue/trigger_action.rs");
        include!("types/glue/trigger_action_notification_property.rs");
        include!("types/glue/trigger_event_batching_condition.rs");
        include!("types/glue/trigger_predicate.rs");
        include!("types/glue/trigger_predicate_condition.rs");
        include!("types/glue/user_defined_function_resource_uri.rs");
        include!("types/glue/get_catalog_table_partition_index.rs");
        include!("types/glue/get_catalog_table_partition_key.rs");
        include!("types/glue/get_catalog_table_storage_descriptor.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_column.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_schema_reference.rs");
        include!(
            "types/glue/get_catalog_table_storage_descriptor_schema_reference_schema_id.rs"
        );
        include!("types/glue/get_catalog_table_storage_descriptor_ser_de_info.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_skewed_info.rs");
        include!("types/glue/get_catalog_table_storage_descriptor_sort_column.rs");
        include!("types/glue/get_catalog_table_target_table.rs");
        include!("types/glue/get_connection_physical_connection_requirement.rs");
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting.rs"
        );
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting_connection_password_encryption.rs"
        );
        include!(
            "types/glue/get_data_catalog_encryption_settings_data_catalog_encryption_setting_encryption_at_rest.rs"
        );
        include!("types/glue/get_script_dag_edge.rs");
        include!("types/glue/get_script_dag_node.rs");
        include!("types/glue/get_script_dag_node_arg.rs");
    }
    pub mod grafana {
        include!("types/grafana/workspace_network_access_control.rs");
        include!("types/grafana/workspace_vpc_configuration.rs");
    }
    pub mod guardduty {
        include!("types/guardduty/detector_datasources.rs");
        include!("types/guardduty/detector_datasources_kubernetes.rs");
        include!("types/guardduty/detector_datasources_kubernetes_audit_logs.rs");
        include!("types/guardduty/detector_datasources_malware_protection.rs");
        include!(
            "types/guardduty/detector_datasources_malware_protection_scan_ec_2_instance_with_findings.rs"
        );
        include!(
            "types/guardduty/detector_datasources_malware_protection_scan_ec_2_instance_with_findings_ebs_volumes.rs"
        );
        include!("types/guardduty/detector_datasources_s_3_logs.rs");
        include!("types/guardduty/detector_feature_additional_configuration.rs");
        include!("types/guardduty/filter_finding_criteria.rs");
        include!("types/guardduty/filter_finding_criteria_criterion.rs");
        include!("types/guardduty/malware_protection_plan_action.rs");
        include!("types/guardduty/malware_protection_plan_action_tagging.rs");
        include!("types/guardduty/malware_protection_plan_protected_resource.rs");
        include!(
            "types/guardduty/malware_protection_plan_protected_resource_s_3_bucket.rs"
        );
        include!("types/guardduty/organization_configuration_datasources.rs");
        include!("types/guardduty/organization_configuration_datasources_kubernetes.rs");
        include!(
            "types/guardduty/organization_configuration_datasources_kubernetes_audit_logs.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection_scan_ec_2_instance_with_findings.rs"
        );
        include!(
            "types/guardduty/organization_configuration_datasources_malware_protection_scan_ec_2_instance_with_findings_ebs_volumes.rs"
        );
        include!("types/guardduty/organization_configuration_datasources_s_3_logs.rs");
        include!(
            "types/guardduty/organization_configuration_feature_additional_configuration.rs"
        );
        include!("types/guardduty/get_detector_feature.rs");
        include!("types/guardduty/get_detector_feature_additional_configuration.rs");
    }
    pub mod iam {
        include!("types/iam/role_inline_policy.rs");
        include!("types/iam/get_access_keys_access_key.rs");
        include!("types/iam/get_group_user.rs");
        include!("types/iam/get_policy_document_statement.rs");
        include!("types/iam/get_policy_document_statement_condition.rs");
        include!("types/iam/get_policy_document_statement_not_principal.rs");
        include!("types/iam/get_policy_document_statement_principal.rs");
        include!("types/iam/get_principal_policy_simulation_context.rs");
        include!("types/iam/get_principal_policy_simulation_result.rs");
        include!(
            "types/iam/get_principal_policy_simulation_result_matched_statement.rs"
        );
        include!("types/iam/get_role_role_last_used.rs");
    }
    pub mod identitystore {
        include!("types/identitystore/group_external_id.rs");
        include!("types/identitystore/user_addresses.rs");
        include!("types/identitystore/user_emails.rs");
        include!("types/identitystore/user_external_id.rs");
        include!("types/identitystore/user_name.rs");
        include!("types/identitystore/user_phone_numbers.rs");
        include!("types/identitystore/get_group_alternate_identifier.rs");
        include!("types/identitystore/get_group_alternate_identifier_external_id.rs");
        include!(
            "types/identitystore/get_group_alternate_identifier_unique_attribute.rs"
        );
        include!("types/identitystore/get_group_external_id.rs");
        include!("types/identitystore/get_group_filter.rs");
        include!("types/identitystore/get_groups_group.rs");
        include!("types/identitystore/get_groups_group_external_id.rs");
        include!("types/identitystore/get_user_address.rs");
        include!("types/identitystore/get_user_alternate_identifier.rs");
        include!("types/identitystore/get_user_alternate_identifier_external_id.rs");
        include!(
            "types/identitystore/get_user_alternate_identifier_unique_attribute.rs"
        );
        include!("types/identitystore/get_user_email.rs");
        include!("types/identitystore/get_user_external_id.rs");
        include!("types/identitystore/get_user_filter.rs");
        include!("types/identitystore/get_user_name.rs");
        include!("types/identitystore/get_user_phone_number.rs");
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
