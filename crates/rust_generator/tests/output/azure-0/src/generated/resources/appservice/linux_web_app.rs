/// Manages a Linux Web App.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleLinuxWebApp = linux_web_app::create(
///         "exampleLinuxWebApp",
///         LinuxWebAppArgs::builder()
///             .location("${exampleServicePlan.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .service_plan_id("${exampleServicePlan.id}")
///             .site_config(LinuxWebAppSiteConfig::builder().build_struct())
///             .build_struct(),
///     );
///     let exampleServicePlan = service_plan::create(
///         "exampleServicePlan",
///         ServicePlanArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .os_type("Linux")
///             .resource_group_name("${example.name}")
///             .sku_name("P1v2")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Linux Web Apps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/linuxWebApp:LinuxWebApp example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.Web/sites/site1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod linux_web_app {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LinuxWebAppArgs {
        /// A map of key-value pairs of App Settings.
        #[builder(into, default)]
        pub app_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `auth_settings` block as defined below.
        #[builder(into, default)]
        pub auth_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppAuthSettings>,
        >,
        /// An `auth_settings_v2` block as defined below.
        #[builder(into, default)]
        pub auth_settings_v2: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppAuthSettingsV2>,
        >,
        /// A `backup` block as defined below.
        #[builder(into, default)]
        pub backup: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppBackup>,
        >,
        /// Should Client Affinity be enabled?
        #[builder(into, default)]
        pub client_affinity_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should Client Certificates be enabled?
        #[builder(into, default)]
        pub client_certificate_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        #[builder(into, default)]
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Client Certificate mode. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. This property has no effect when `client_certificate_enabled` is `false`. Defaults to `Required`.
        #[builder(into, default)]
        pub client_certificate_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `connection_string` blocks as defined below.
        #[builder(into, default)]
        pub connection_strings: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::LinuxWebAppConnectionString>>,
        >,
        /// Should the Linux Web App be enabled? Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        #[builder(into, default)]
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Should the Linux Web App require HTTPS connections. Defaults to `false`.
        #[builder(into, default)]
        pub https_only: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity).
        #[builder(into, default)]
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The Azure Region where the Linux Web App should exist. Changing this forces a new Linux Web App to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `logs` block as defined below.
        #[builder(into, default)]
        pub logs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppLogs>,
        >,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Should public network access be enabled for the Web App. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the Resource Group where the Linux Web App should exist. Changing this forces a new Linux Web App to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Service Plan that this Linux App Service will be created in.
        #[builder(into)]
        pub service_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `site_config` block as defined below.
        #[builder(into)]
        pub site_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appservice::LinuxWebAppSiteConfig,
        >,
        /// A `sticky_settings` block as defined below.
        #[builder(into, default)]
        pub sticky_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::appservice::LinuxWebAppStickySettings>,
        >,
        /// One or more `storage_account` blocks as defined below.
        #[builder(into, default)]
        pub storage_accounts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::appservice::LinuxWebAppStorageAccount>>,
        >,
        /// A mapping of tags which should be assigned to the Linux Web App.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub virtual_network_subnet_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        ///
        /// > **NOTE:** Setting this value to true will disable the ability to use `zip_deploy_file` which currently relies on the default publishing profile.
        #[builder(into, default)]
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The local path and filename of the Zip packaged application to deploy to this Linux Web App.
        ///
        /// > **Note:** Using this value requires either `WEBSITE_RUN_FROM_PACKAGE=1` or `SCM_DO_BUILD_DURING_DEPLOYMENT=true` to be set on the App in `app_settings`. Refer to the Azure docs on [running the Web App directly from the Zip package](https://learn.microsoft.com/en-us/azure/app-service/deploy-run-package), or [automating the build for Zip deploy](https://learn.microsoft.com/en-us/azure/app-service/deploy-zip#enable-build-automation-for-zip-deploy) for further details.
        #[builder(into, default)]
        pub zip_deploy_file: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LinuxWebAppResult {
        /// A map of key-value pairs of App Settings.
        pub app_settings: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `auth_settings` block as defined below.
        pub auth_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppAuthSettings>,
        >,
        /// An `auth_settings_v2` block as defined below.
        pub auth_settings_v2: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppAuthSettingsV2>,
        >,
        /// A `backup` block as defined below.
        pub backup: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppBackup>,
        >,
        /// Should Client Affinity be enabled?
        pub client_affinity_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should Client Certificates be enabled?
        pub client_certificate_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Paths to exclude when using client certificates, separated by ;
        pub client_certificate_exclusion_paths: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// The Client Certificate mode. Possible values are `Required`, `Optional`, and `OptionalInteractiveUser`. This property has no effect when `client_certificate_enabled` is `false`. Defaults to `Required`.
        pub client_certificate_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// One or more `connection_string` blocks as defined below.
        pub connection_strings: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appservice::LinuxWebAppConnectionString>>,
        >,
        /// The identifier used by App Service to perform domain ownership verification via DNS TXT record.
        pub custom_domain_verification_id: pulumi_gestalt_rust::Output<String>,
        /// The default hostname of the Linux Web App.
        pub default_hostname: pulumi_gestalt_rust::Output<String>,
        /// Should the Linux Web App be enabled? Defaults to `true`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Should the default FTP Basic Authentication publishing profile be enabled. Defaults to `true`.
        pub ftp_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The ID of the App Service Environment used by App Service.
        pub hosting_environment_id: pulumi_gestalt_rust::Output<String>,
        /// Should the Linux Web App require HTTPS connections. Defaults to `false`.
        pub https_only: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppIdentity>,
        >,
        /// The User Assigned Identity ID used for accessing KeyVault secrets. The identity must be assigned to the application in the `identity` block. [For more information see - Access vaults with a user-assigned identity](https://docs.microsoft.com/azure/app-service/app-service-key-vault-references#access-vaults-with-a-user-assigned-identity).
        pub key_vault_reference_identity_id: pulumi_gestalt_rust::Output<String>,
        /// The Kind value for this Linux Web App.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Linux Web App should exist. Changing this forces a new Linux Web App to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// A `logs` block as defined below.
        pub logs: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppLogs>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A list of outbound IP addresses - such as `["52.23.25.3", "52.143.43.12"]`
        pub outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12`.
        pub outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// A list of possible outbound ip address.
        pub possible_outbound_ip_address_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A comma-separated list of outbound IP addresses - such as `52.23.25.3,52.143.43.12,52.143.43.17` - not all of which are necessarily in use. Superset of `outbound_ip_addresses`.
        pub possible_outbound_ip_addresses: pulumi_gestalt_rust::Output<String>,
        /// Should public network access be enabled for the Web App. Defaults to `true`.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the Resource Group where the Linux Web App should exist. Changing this forces a new Linux Web App to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Service Plan that this Linux App Service will be created in.
        pub service_plan_id: pulumi_gestalt_rust::Output<String>,
        /// A `site_config` block as defined below.
        pub site_config: pulumi_gestalt_rust::Output<
            super::super::types::appservice::LinuxWebAppSiteConfig,
        >,
        /// A `site_credential` block as defined below.
        pub site_credentials: pulumi_gestalt_rust::Output<
            Vec<super::super::types::appservice::LinuxWebAppSiteCredential>,
        >,
        /// A `sticky_settings` block as defined below.
        pub sticky_settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::appservice::LinuxWebAppStickySettings>,
        >,
        /// One or more `storage_account` blocks as defined below.
        pub storage_accounts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::appservice::LinuxWebAppStorageAccount>>,
        >,
        /// A mapping of tags which should be assigned to the Linux Web App.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub virtual_network_subnet_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Should the default WebDeploy Basic Authentication publishing credentials enabled. Defaults to `true`.
        ///
        /// > **NOTE:** Setting this value to true will disable the ability to use `zip_deploy_file` which currently relies on the default publishing profile.
        pub webdeploy_publish_basic_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The local path and filename of the Zip packaged application to deploy to this Linux Web App.
        ///
        /// > **Note:** Using this value requires either `WEBSITE_RUN_FROM_PACKAGE=1` or `SCM_DO_BUILD_DURING_DEPLOYMENT=true` to be set on the App in `app_settings`. Refer to the Azure docs on [running the Web App directly from the Zip package](https://learn.microsoft.com/en-us/azure/app-service/deploy-run-package), or [automating the build for Zip deploy](https://learn.microsoft.com/en-us/azure/app-service/deploy-zip#enable-build-automation-for-zip-deploy) for further details.
        pub zip_deploy_file: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LinuxWebAppArgs,
    ) -> LinuxWebAppResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_settings_binding = args.app_settings.get_output(context);
        let auth_settings_binding = args.auth_settings.get_output(context);
        let auth_settings_v2_binding = args.auth_settings_v2.get_output(context);
        let backup_binding = args.backup.get_output(context);
        let client_affinity_enabled_binding = args
            .client_affinity_enabled
            .get_output(context);
        let client_certificate_enabled_binding = args
            .client_certificate_enabled
            .get_output(context);
        let client_certificate_exclusion_paths_binding = args
            .client_certificate_exclusion_paths
            .get_output(context);
        let client_certificate_mode_binding = args
            .client_certificate_mode
            .get_output(context);
        let connection_strings_binding = args.connection_strings.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let ftp_publish_basic_authentication_enabled_binding = args
            .ftp_publish_basic_authentication_enabled
            .get_output(context);
        let https_only_binding = args.https_only.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let key_vault_reference_identity_id_binding = args
            .key_vault_reference_identity_id
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let logs_binding = args.logs.get_output(context);
        let name_binding = args.name.get_output(context);
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let service_plan_id_binding = args.service_plan_id.get_output(context);
        let site_config_binding = args.site_config.get_output(context);
        let sticky_settings_binding = args.sticky_settings.get_output(context);
        let storage_accounts_binding = args.storage_accounts.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let virtual_network_subnet_id_binding = args
            .virtual_network_subnet_id
            .get_output(context);
        let webdeploy_publish_basic_authentication_enabled_binding = args
            .webdeploy_publish_basic_authentication_enabled
            .get_output(context);
        let zip_deploy_file_binding = args.zip_deploy_file.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/linuxWebApp:LinuxWebApp".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appSettings".into(),
                    value: &app_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSettings".into(),
                    value: &auth_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authSettingsV2".into(),
                    value: &auth_settings_v2_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backup".into(),
                    value: &backup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientAffinityEnabled".into(),
                    value: &client_affinity_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateEnabled".into(),
                    value: &client_certificate_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateExclusionPaths".into(),
                    value: &client_certificate_exclusion_paths_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientCertificateMode".into(),
                    value: &client_certificate_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionStrings".into(),
                    value: &connection_strings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ftpPublishBasicAuthenticationEnabled".into(),
                    value: &ftp_publish_basic_authentication_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "httpsOnly".into(),
                    value: &https_only_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "keyVaultReferenceIdentityId".into(),
                    value: &key_vault_reference_identity_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logs".into(),
                    value: &logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "servicePlanId".into(),
                    value: &service_plan_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "siteConfig".into(),
                    value: &site_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stickySettings".into(),
                    value: &sticky_settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageAccounts".into(),
                    value: &storage_accounts_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualNetworkSubnetId".into(),
                    value: &virtual_network_subnet_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "webdeployPublishBasicAuthenticationEnabled".into(),
                    value: &webdeploy_publish_basic_authentication_enabled_binding
                        .drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zipDeployFile".into(),
                    value: &zip_deploy_file_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LinuxWebAppResult {
            app_settings: o.get_field("appSettings"),
            auth_settings: o.get_field("authSettings"),
            auth_settings_v2: o.get_field("authSettingsV2"),
            backup: o.get_field("backup"),
            client_affinity_enabled: o.get_field("clientAffinityEnabled"),
            client_certificate_enabled: o.get_field("clientCertificateEnabled"),
            client_certificate_exclusion_paths: o
                .get_field("clientCertificateExclusionPaths"),
            client_certificate_mode: o.get_field("clientCertificateMode"),
            connection_strings: o.get_field("connectionStrings"),
            custom_domain_verification_id: o.get_field("customDomainVerificationId"),
            default_hostname: o.get_field("defaultHostname"),
            enabled: o.get_field("enabled"),
            ftp_publish_basic_authentication_enabled: o
                .get_field("ftpPublishBasicAuthenticationEnabled"),
            hosting_environment_id: o.get_field("hostingEnvironmentId"),
            https_only: o.get_field("httpsOnly"),
            identity: o.get_field("identity"),
            key_vault_reference_identity_id: o.get_field("keyVaultReferenceIdentityId"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            logs: o.get_field("logs"),
            name: o.get_field("name"),
            outbound_ip_address_lists: o.get_field("outboundIpAddressLists"),
            outbound_ip_addresses: o.get_field("outboundIpAddresses"),
            possible_outbound_ip_address_lists: o
                .get_field("possibleOutboundIpAddressLists"),
            possible_outbound_ip_addresses: o.get_field("possibleOutboundIpAddresses"),
            public_network_access_enabled: o.get_field("publicNetworkAccessEnabled"),
            resource_group_name: o.get_field("resourceGroupName"),
            service_plan_id: o.get_field("servicePlanId"),
            site_config: o.get_field("siteConfig"),
            site_credentials: o.get_field("siteCredentials"),
            sticky_settings: o.get_field("stickySettings"),
            storage_accounts: o.get_field("storageAccounts"),
            tags: o.get_field("tags"),
            virtual_network_subnet_id: o.get_field("virtualNetworkSubnetId"),
            webdeploy_publish_basic_authentication_enabled: o
                .get_field("webdeployPublishBasicAuthenticationEnabled"),
            zip_deploy_file: o.get_field("zipDeployFile"),
        }
    }
}
