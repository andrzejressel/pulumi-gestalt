#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_linux_function_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetLinuxFunctionAppArgs {
        /// The name which should be used for this Linux Function App.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Linux Function App should exist.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetLinuxFunctionAppResult {
        /// A map of key-value pairs for [App Settings](https://docs.microsoft.com/azure/azure-functions/functions-app-settings) and custom values.
        pub app_settings: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppAuthSetting>,
        >,
        /// A `auth_settings_v2` block as defined below.
        pub auth_settings_v2s: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppAuthSettingsV2,
            >,
        >,
        /// The current availability state. Possible values are `Normal`, `Limited`, and `DisasterRecoveryMode`.
        pub availability: pulumi_gestalt_rust::Output<String>,
        /// A `backup` block as defined below.
        pub backups: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppBackup>,
        >,
        /// Is built in logging enabled?
        pub builtin_logging_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Are Client Certificates enabled?
        pub client_certificate_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::Output<String>,
        /// The mode of the Function App's client certificates requirement for incoming requests.
        pub client_certificate_mode: pulumi_gestalt_rust::Output<String>,
        /// A `connection_string` blocks as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppConnectionString,
            >,
        >,
        /// Are the settings for linking the Function App to storage suppressed?
        pub content_share_force_disabled: pulumi_gestalt_rust::Output<bool>,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The amount of memory in gigabyte-seconds that your application is allowed to consume per day.
        pub daily_memory_time_quota: pulumi_gestalt_rust::Output<i32>,
        /// The default hostname of the Linux Function App.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// Is this backup job enabled?
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Are the default FTP Basic Authentication publishing credentials enabled.
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The runtime version associated with the Function App.
        pub functions_extension_version: pulumi_gestalt_rust::Output<String>,
        /// The ID of the App Service Environment used by Function App.
        pub hosting_environment_id: pulumi_gestalt_rust::Output<String>,
        /// Can the Function App only be accessed via HTTPS?
        pub https_only: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppIdentity>,
        >,
        /// The Kind value for this Linux Function App.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Linux Function App exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Site Credentials Username used for publishing.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of outbound IP addresses. For example `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A list of possible outbound IP addresses, not all of which are necessarily in use. This is a superset of `outbound_ip_address_list`. For example `["52.23.25.3", "52.143.43.12"]`.
        pub possible_outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of possible outbound IP addresses as a string. For example `52.23.25.3,52.143.43.12,52.143.43.17`. This is a superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// Is Public Network Access enabled for this Linux Function App.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the App Service Plan within which this Function App has been created.
        pub service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppSiteConfig>,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::appservice::GetLinuxFunctionAppSiteCredential,
            >,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appservice::GetLinuxFunctionAppStickySetting>,
        >,
        /// The access key used to access the backend storage account for the Function App.
        pub storage_account_access_key: pulumi_gestalt_rust::Output<String>,
        /// The backend storage account name used by this Function App.
        pub storage_account_name: pulumi_gestalt_rust::Output<String>,
        /// The Key Vault Secret ID, including version, that contains the Connection String to connect to the storage account for this Function App.
        pub storage_key_vault_secret_id: pulumi_gestalt_rust::Output<String>,
        /// Does the Function App use Managed Identity to access the storage account?
        pub storage_uses_managed_identity: pulumi_gestalt_rust::Output<bool>,
        /// A mapping of tags which are assigned to the Linux Function App.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The current usage state. Possible values are `Normal` and `Exceeded`.
        pub usage: pulumi_gestalt_rust::Output<String>,
        /// The Virtual Network Subnet ID used for this IP Restriction.
        pub virtual_network_subnet_id: pulumi_gestalt_rust::Output<String>,
        /// Are the default WebDeploy Basic Authentication publishing credentials enabled.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            bool,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetLinuxFunctionAppArgs,
    ) -> GetLinuxFunctionAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:appservice/getLinuxFunctionApp:getLinuxFunctionApp".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetLinuxFunctionAppResult {
            app_settings: o.get_field("appSettings"),
            auth_settings: o.get_field("authSettings"),
            auth_settings_v2s: o.get_field("authSettingsV2s"),
            availability: o.get_field("availability"),
            backups: o.get_field("backups"),
            builtin_logging_enabled: o.get_field("builtinLoggingEnabled"),
            client_certificate_enabled: o.get_field("clientCertificateEnabled"),
            client_certificate_exclusion_paths: o
                .get_field("clientCertificateExclusionPaths"),
            client_certificate_mode: o.get_field("clientCertificateMode"),
            connection_strings: o.get_field("connectionStrings"),
            content_share_force_disabled: o.get_field("contentShareForceDisabled"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            daily_memory_time_quota: o.get_field("dailyMemoryTimeQuota"),
            default_hostname: o.get_field("defaultHostname"),
            enabled: o.get_field("enabled"),
            ftp_publish_basic_authentication_enabled: o
                .get_field("ftpPublishBasicAuthenticationEnabled"),
            functions_extension_version: o.get_field("functionsExtensionVersion"),
            hosting_environment_id: o.get_field("hostingEnvironmentId"),
            https_only: o.get_field("httpsOnly"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            outbound_ip_address_lists: o.get_field("outboundIpAddressLists"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_address_lists: o
                .get_field("possibleOutboundIpAddressLists"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_plan_id: o.get_field("servicePlanId"),
            site_configs: o.get_field("siteConfigs"),
            site_credentials: o.get_field("siteCredentials"),
            sticky_settings: o.get_field("stickySettings"),
            storage_account_access_key: o.get_field("storageAccountAccessKey"),
            storage_account_name: o.get_field("storageAccountName"),
            storage_key_vault_secret_id: o.get_field("storageKeyVaultSecretId"),
            storage_uses_managed_identity: o.get_field("storageUsesManagedIdentity"),
            tags: o.get_field("tags"),
            usage: o.get_field("usage"),
            virtual_network_subnet_id: o.get_field("virtualNetworkSubnetId"),
            webdeploy_publish_basic_authentication_enabled: o
                .get_field("webdeployPublishBasicAuthenticationEnabled"),
        }
    }
}
