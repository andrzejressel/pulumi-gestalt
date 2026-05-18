#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsFunctionAppSiteConfig {
    /// If this Windows Function App is Always On enabled. Defaults to `false`.
    /// 
    /// > **NOTE:** when running in a Consumption or Premium Plan, `always_on` feature should be turned off. Please turn it off before upgrading the service plan from standard to premium.
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
    /// The App command line to launch.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Option<String>,
    /// The number of workers this function app can scale out to. Only applicable to apps on the Consumption and Premium plan.
    #[builder(into)]
    #[serde(rename = "appScaleLimit")]
    pub r#app_scale_limit: Option<i32>,
    /// An `app_service_logs` block as defined above.
    #[builder(into)]
    #[serde(rename = "appServiceLogs")]
    pub r#app_service_logs: Option<Box<super::super::types::appservice::WindowsFunctionAppSiteConfigAppServiceLogs>>,
    /// The Connection String for linking the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsConnectionString")]
    pub r#application_insights_connection_string: Option<String>,
    /// The Instrumentation Key for connecting the Windows Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsKey")]
    pub r#application_insights_key: Option<String>,
    /// An `application_stack` block as defined above.
    /// 
    /// > **Note:** If this is set, there must not be an application setting `FUNCTIONS_WORKER_RUNTIME`.
    #[builder(into)]
    #[serde(rename = "applicationStack")]
    pub r#application_stack: Option<Box<super::super::types::appservice::WindowsFunctionAppSiteConfigApplicationStack>>,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Option<Box<super::super::types::appservice::WindowsFunctionAppSiteConfigCors>>,
    /// Specifies a list of Default Documents for the Windows Function App.
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
    /// State of FTP / FTPS service for this Windows Function App. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Only valid in conjunction with `health_check_path`.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: Option<i32>,
    /// The path to be checked for this Windows Function App health.
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
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::WindowsFunctionAppSiteConfigIpRestriction>>,
    /// The Site load balancing mode. Possible values include: `WeightedRoundRobin`, `LeastRequests`, `LeastResponseTime`, `WeightedTotalTraffic`, `RequestHash`, `PerSiteRoundRobin`. Defaults to `LeastRequests` if omitted.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Option<String>,
    /// Managed pipeline mode. Possible values include: `Integrated`, `Classic`. Defaults to `Integrated`.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Option<String>,
    /// Configures the minimum version of TLS required for SSL requests. Possible values include: `1.0`, `1.1`, `1.2` and `1.3`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: Option<String>,
    /// The number of pre-warmed instances for this Windows Function App. Only affects apps on an Elastic Premium plan.
    #[builder(into)]
    #[serde(rename = "preWarmedInstanceCount")]
    pub r#pre_warmed_instance_count: Option<i32>,
    /// Should Remote Debugging be enabled. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Option<bool>,
    /// The Remote Debugging Version. Possible values include `VS2017`, `VS2019`, and `VS2022`.
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
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::WindowsFunctionAppSiteConfigScmIpRestriction>>,
    /// Configures the minimum version of TLS required for SSL requests to the SCM site. Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
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
    /// Should the Windows Function App use a 32-bit worker process. Defaults to `true`.
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsFunctionAppSiteConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "always_on",
                    &self.r#always_on,
                ),
                to_pulumi_object_field(
                    "api_definition_url",
                    &self.r#api_definition_url,
                ),
                to_pulumi_object_field(
                    "api_management_api_id",
                    &self.r#api_management_api_id,
                ),
                to_pulumi_object_field(
                    "app_command_line",
                    &self.r#app_command_line,
                ),
                to_pulumi_object_field(
                    "app_scale_limit",
                    &self.r#app_scale_limit,
                ),
                to_pulumi_object_field(
                    "app_service_logs",
                    &self.r#app_service_logs,
                ),
                to_pulumi_object_field(
                    "application_insights_connection_string",
                    &self.r#application_insights_connection_string,
                ),
                to_pulumi_object_field(
                    "application_insights_key",
                    &self.r#application_insights_key,
                ),
                to_pulumi_object_field(
                    "application_stack",
                    &self.r#application_stack,
                ),
                to_pulumi_object_field(
                    "cors",
                    &self.r#cors,
                ),
                to_pulumi_object_field(
                    "default_documents",
                    &self.r#default_documents,
                ),
                to_pulumi_object_field(
                    "detailed_error_logging_enabled",
                    &self.r#detailed_error_logging_enabled,
                ),
                to_pulumi_object_field(
                    "elastic_instance_minimum",
                    &self.r#elastic_instance_minimum,
                ),
                to_pulumi_object_field(
                    "ftps_state",
                    &self.r#ftps_state,
                ),
                to_pulumi_object_field(
                    "health_check_eviction_time_in_min",
                    &self.r#health_check_eviction_time_in_min,
                ),
                to_pulumi_object_field(
                    "health_check_path",
                    &self.r#health_check_path,
                ),
                to_pulumi_object_field(
                    "http_2_enabled",
                    &self.r#http_2_enabled,
                ),
                to_pulumi_object_field(
                    "ip_restriction_default_action",
                    &self.r#ip_restriction_default_action,
                ),
                to_pulumi_object_field(
                    "ip_restrictions",
                    &self.r#ip_restrictions,
                ),
                to_pulumi_object_field(
                    "load_balancing_mode",
                    &self.r#load_balancing_mode,
                ),
                to_pulumi_object_field(
                    "managed_pipeline_mode",
                    &self.r#managed_pipeline_mode,
                ),
                to_pulumi_object_field(
                    "minimum_tls_version",
                    &self.r#minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "pre_warmed_instance_count",
                    &self.r#pre_warmed_instance_count,
                ),
                to_pulumi_object_field(
                    "remote_debugging_enabled",
                    &self.r#remote_debugging_enabled,
                ),
                to_pulumi_object_field(
                    "remote_debugging_version",
                    &self.r#remote_debugging_version,
                ),
                to_pulumi_object_field(
                    "runtime_scale_monitoring_enabled",
                    &self.r#runtime_scale_monitoring_enabled,
                ),
                to_pulumi_object_field(
                    "scm_ip_restriction_default_action",
                    &self.r#scm_ip_restriction_default_action,
                ),
                to_pulumi_object_field(
                    "scm_ip_restrictions",
                    &self.r#scm_ip_restrictions,
                ),
                to_pulumi_object_field(
                    "scm_minimum_tls_version",
                    &self.r#scm_minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "scm_type",
                    &self.r#scm_type,
                ),
                to_pulumi_object_field(
                    "scm_use_main_ip_restriction",
                    &self.r#scm_use_main_ip_restriction,
                ),
                to_pulumi_object_field(
                    "use_32_bit_worker",
                    &self.r#use_32_bit_worker,
                ),
                to_pulumi_object_field(
                    "vnet_route_all_enabled",
                    &self.r#vnet_route_all_enabled,
                ),
                to_pulumi_object_field(
                    "websockets_enabled",
                    &self.r#websockets_enabled,
                ),
                to_pulumi_object_field(
                    "windows_fx_version",
                    &self.r#windows_fx_version,
                ),
                to_pulumi_object_field(
                    "worker_count",
                    &self.r#worker_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsFunctionAppSiteConfig {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#always_on: {
                        let field_value = match fields_map.get("always_on") {
                            Some(value) => value,
                            None => bail!("Missing field 'always_on' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_definition_url: {
                        let field_value = match fields_map.get("api_definition_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_definition_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_management_api_id: {
                        let field_value = match fields_map.get("api_management_api_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_management_api_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_command_line: {
                        let field_value = match fields_map.get("app_command_line") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_command_line' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_scale_limit: {
                        let field_value = match fields_map.get("app_scale_limit") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_scale_limit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_service_logs: {
                        let field_value = match fields_map.get("app_service_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_service_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_insights_connection_string: {
                        let field_value = match fields_map.get("application_insights_connection_string") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_insights_connection_string' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_insights_key: {
                        let field_value = match fields_map.get("application_insights_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_insights_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_stack: {
                        let field_value = match fields_map.get("application_stack") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_stack' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors: {
                        let field_value = match fields_map.get("cors") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_documents: {
                        let field_value = match fields_map.get("default_documents") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_documents' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_error_logging_enabled: {
                        let field_value = match fields_map.get("detailed_error_logging_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailed_error_logging_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#elastic_instance_minimum: {
                        let field_value = match fields_map.get("elastic_instance_minimum") {
                            Some(value) => value,
                            None => bail!("Missing field 'elastic_instance_minimum' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ftps_state: {
                        let field_value = match fields_map.get("ftps_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'ftps_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_eviction_time_in_min: {
                        let field_value = match fields_map.get("health_check_eviction_time_in_min") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check_eviction_time_in_min' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_path: {
                        let field_value = match fields_map.get("health_check_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_enabled: {
                        let field_value = match fields_map.get("http_2_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_restriction_default_action: {
                        let field_value = match fields_map.get("ip_restriction_default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_restriction_default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_restrictions: {
                        let field_value = match fields_map.get("ip_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancing_mode: {
                        let field_value = match fields_map.get("load_balancing_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancing_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_pipeline_mode: {
                        let field_value = match fields_map.get("managed_pipeline_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'managed_pipeline_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_tls_version: {
                        let field_value = match fields_map.get("minimum_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_warmed_instance_count: {
                        let field_value = match fields_map.get("pre_warmed_instance_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_warmed_instance_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_debugging_enabled: {
                        let field_value = match fields_map.get("remote_debugging_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_debugging_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_debugging_version: {
                        let field_value = match fields_map.get("remote_debugging_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_debugging_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime_scale_monitoring_enabled: {
                        let field_value = match fields_map.get("runtime_scale_monitoring_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime_scale_monitoring_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_ip_restriction_default_action: {
                        let field_value = match fields_map.get("scm_ip_restriction_default_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_ip_restriction_default_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_ip_restrictions: {
                        let field_value = match fields_map.get("scm_ip_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_ip_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_minimum_tls_version: {
                        let field_value = match fields_map.get("scm_minimum_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_minimum_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_type: {
                        let field_value = match fields_map.get("scm_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_use_main_ip_restriction: {
                        let field_value = match fields_map.get("scm_use_main_ip_restriction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_use_main_ip_restriction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_32_bit_worker: {
                        let field_value = match fields_map.get("use_32_bit_worker") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_32_bit_worker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_route_all_enabled: {
                        let field_value = match fields_map.get("vnet_route_all_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnet_route_all_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#websockets_enabled: {
                        let field_value = match fields_map.get("websockets_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'websockets_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#windows_fx_version: {
                        let field_value = match fields_map.get("windows_fx_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'windows_fx_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_count: {
                        let field_value = match fields_map.get("worker_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'worker_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
