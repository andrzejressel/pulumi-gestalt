#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSlotSiteConfig {
    /// If this Linux Web App is Always On enabled. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Option<bool>,
    /// The URL to the API Definition for this Linux Web App Slot.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Option<String>,
    /// The API Management API ID this Linux Web App Slot is associated with.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: Option<String>,
    /// The App command line to launch.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Option<String>,
    /// A `application_stack` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationStack")]
    pub r#application_stack: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigApplicationStack>>,
    /// A `auto_heal_setting` block as defined above. Required with `auto_heal`.
    #[builder(into)]
    #[serde(rename = "autoHealSetting")]
    pub r#auto_heal_setting: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSetting>>,
    /// The Linux Web App Slot Name to automatically swap to when deployment to that slot is successfully completed.
    /// 
    /// > **Note:** This must be a valid slot name on the target Linux Web App.
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Option<String>,
    /// The Client ID of the Managed Service Identity to use for connections to the Azure Container Registry.
    #[builder(into)]
    #[serde(rename = "containerRegistryManagedIdentityClientId")]
    pub r#container_registry_managed_identity_client_id: Option<String>,
    /// Should connections for Azure Container Registry use Managed Identity.
    #[builder(into)]
    #[serde(rename = "containerRegistryUseManagedIdentity")]
    pub r#container_registry_use_managed_identity: Option<bool>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigCors>>,
    /// Specifies a list of Default Documents for the Linux Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Only valid in conjunction with `health_check_path`.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Option<i32>,
    /// The path to the Health Check.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Option<String>,
    /// Should the HTTP2 be enabled?
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Option<bool>,
    /// The Default action for traffic that does not match any `ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: Option<String>,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigIpRestriction>>,
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Option<String>,
    /// The Site load balancing. Possible values include: `WeightedRoundRobin`, `LeastRequests`, `LeastResponseTime`, `WeightedTotalTraffic`, `RequestHash`, `PerSiteRoundRobin`. Defaults to `LeastRequests` if omitted.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Option<String>,
    /// Use Local MySQL. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Option<bool>,
    /// Managed pipeline mode. Possible values include: `Integrated`, `Classic`. Defaults to `Integrated`.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Option<String>,
    /// The configures the minimum version of TLS required for SSL requests. Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Option<String>,
    /// Should Remote Debugging be enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Option<bool>,
    /// The Remote Debugging Version. Possible values include `VS2017`, `VS2019` and `VS2022`
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Option<String>,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: Option<String>,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigScmIpRestriction>>,
    /// The configures the minimum version of TLS required for SSL requests to the SCM site Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Option<String>,
    /// Should the Linux Web App `ip_restriction` configuration be used for the SCM also.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the Linux Web App use a 32-bit worker? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: Option<bool>,
    /// Should all outbound traffic have NAT Gateways, Network Security Groups and User Defined Routes applied? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Option<bool>,
    /// Should Web Sockets be enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Option<bool>,
    /// The number of Workers for this Linux App Service Slot.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Option<i32>,
}
