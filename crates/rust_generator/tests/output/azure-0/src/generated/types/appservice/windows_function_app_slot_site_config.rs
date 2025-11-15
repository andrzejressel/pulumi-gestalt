#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFunctionAppSlotSiteConfig {
    /// If this Windows Web App is Always On enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Option<bool>,
    /// The URL of the API definition that describes this Windows Function App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Option<String>,
    /// The ID of the API Management API for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: Option<String>,
    /// The program and any arguments used to launch this app via the command line. (Example `node myapp.js`).
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Option<String>,
    /// The number of workers this function app can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Option<i32>,
    /// an `app_service_logs` block as detailed below.
    #[builder(into)]
    #[serde(rename = "appServiceLogs")]
    pub r#app_service_logs: Option<Box<super::super::types::appservice::WindowsFunctionAppSlotSiteConfigAppServiceLogs>>,
    /// The Connection String for linking the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsConnectionString")]
    pub r#application_insights_connection_string: Option<String>,
    /// The Instrumentation Key for connecting the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsKey")]
    pub r#application_insights_key: Option<String>,
    /// an `application_stack` block as detailed below.
    #[builder(into)]
    #[serde(rename = "applicationStack")]
    pub r#application_stack: Option<Box<super::super::types::appservice::WindowsFunctionAppSlotSiteConfigApplicationStack>>,
    /// The name of the slot to automatically swap with when this slot is successfully deployed.
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Option<String>,
    /// a `cors` block as detailed below.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Option<Box<super::super::types::appservice::WindowsFunctionAppSlotSiteConfigCors>>,
    /// Specifies a list of Default Documents for the Windows Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Option<Vec<String>>,
    /// Is detailed error logging enabled
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Option<bool>,
    /// The number of minimum instances for this Windows Function App. Only affects apps on Elastic Premium plans.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Option<i32>,
    /// State of FTP / FTPS service for this function app. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// The amount of time in minutes that a node is unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Defaults to `0`. Only valid in conjunction with `health_check_path`.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Option<i32>,
    /// The path to be checked for this function app health.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Option<String>,
    /// Specifies if the HTTP2 protocol should be enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Option<bool>,
    /// The Default action for traffic that does not match any `ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: Option<String>,
    /// an `ip_restriction` block as detailed below.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::WindowsFunctionAppSlotSiteConfigIpRestriction>>,
    /// The Site load balancing mode. Possible values include: `WeightedRoundRobin`, `LeastRequests`, `LeastResponseTime`, `WeightedTotalTraffic`, `RequestHash`, `PerSiteRoundRobin`. Defaults to `LeastRequests` if omitted.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Option<String>,
    /// The Managed Pipeline mode. Possible values include: `Integrated`, `Classic`. Defaults to `Integrated`.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Option<String>,
    /// The configures the minimum version of TLS required for SSL requests. Possible values include: `1.0`, `1.1`, `1.2` and `1.3`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Option<String>,
    /// The number of pre-warmed instances for this function app. Only affects apps on an Elastic Premium plan.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Option<i32>,
    /// Should Remote Debugging be enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Option<bool>,
    /// The Remote Debugging Version. Possible values include `VS2017`, `VS2019`, and `VS2022`
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Option<String>,
    /// Should Scale Monitoring of the Functions Runtime be enabled?
    /// 
    /// > **NOTE:** Functions runtime scale monitoring can only be enabled for Elastic Premium Function Apps or Workflow Standard Logic Apps and requires a minimum prewarmed instance count of 1.
    #[builder(into)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: Option<bool>,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule. possible values include `Allow` and `Deny`. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: Option<String>,
    /// a `scm_ip_restriction` block as detailed below.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::WindowsFunctionAppSlotSiteConfigScmIpRestriction>>,
    /// Configures the minimum version of TLS required for SSL requests to the SCM site Possible values include: `1.0`, `1.1`, `1.2` and `1.3`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Option<String>,
    /// The SCM Type in use by the Windows Function App.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Option<String>,
    /// Should the Windows Function App `ip_restriction` configuration be used for the SCM also.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the Windows Web App use a 32-bit worker. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: Option<bool>,
    /// Should all outbound traffic to have NAT Gateways, Network Security Groups and User Defined Routes applied? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Option<bool>,
    /// Should Web Sockets be enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Option<bool>,
    /// The Windows FX Version string.
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Option<String>,
    /// The number of Workers for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Option<i32>,
}
