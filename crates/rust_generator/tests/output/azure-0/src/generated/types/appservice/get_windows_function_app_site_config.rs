#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppSiteConfig {
    /// Is this Windows Function App Always On?.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: bool,
    /// The URL of the API definition that describes this Windows Function App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: String,
    /// The ID of the API Management API for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: String,
    /// The App command line to launch.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: String,
    /// The number of workers this function app can scale out to.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: i32,
    /// A `app_service_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "appServiceLogs")]
    pub r#app_service_logs: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigAppServiceLog>,
    /// The Connection String for linking the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsConnectionString")]
    pub r#application_insights_connection_string: String,
    /// The Instrumentation Key for connecting the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsKey")]
    pub r#application_insights_key: String,
    /// A `application_stack` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationStacks")]
    pub r#application_stacks: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigApplicationStack>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigCor>,
    /// A list of Default Documents for the Windows Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Vec<String>,
    /// Is detailed error logging enabled?
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: bool,
    /// The number of minimum instances for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: i32,
    /// State of FTP / FTPS service for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: String,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: i32,
    /// The path to be checked for this Windows Function App health.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: String,
    /// Is the HTTP2 protocol enabled?
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: bool,
    /// The Default action for traffic that does not match any `ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: String,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigIpRestriction>,
    /// The Site load balancing mode.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: String,
    /// The Managed pipeline mode.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: String,
    /// The minimum version of TLS required for SSL requests.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: String,
    /// The number of pre-warmed instances for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: i32,
    /// Is Remote Debugging enabled?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: bool,
    /// The Remote Debugging Version.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: String,
    /// Is Scale Monitoring of the Functions Runtime enabled?
    #[builder(into)]
    #[serde(rename = "runtimeScaleMonitoringEnabled")]
    pub r#runtime_scale_monitoring_enabled: bool,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: String,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigScmIpRestriction>,
    /// The minimum version of TLS required for SSL requests to the SCM site.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: String,
    /// The SCM type.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: String,
    /// Is the `ip_restriction` configuration used for the SCM?.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: bool,
    /// Is the Windows Function App using a 32-bit worker process?
    #[builder(into)]
    #[serde(rename = "use32BitWorker")]
    pub r#use_32_bit_worker: bool,
    /// Are all outbound traffic to NAT Gateways, Network Security Groups and User Defined Routes applied?
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: bool,
    /// Are Web Sockets enabled?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: bool,
    /// The Windows FX version.
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: String,
    /// The number of Workers for this Windows Function App.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: i32,
}
