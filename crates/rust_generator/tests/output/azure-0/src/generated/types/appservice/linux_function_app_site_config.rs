#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxFunctionAppSiteConfig {
    /// If this Linux Web App is Always On enabled. Defaults to `false`.
    /// 
    /// > **NOTE:** when running in a Consumption or Premium Plan, `always_on` feature should be turned off. Please turn it off before upgrading the service plan from standard to premium.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Option<bool>,
    /// The URL of the API definition that describes this Linux Function App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: Option<String>,
    /// The ID of the API Management API for this Linux Function App.
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
    pub r#app_service_logs: Option<Box<super::super::types::appservice::LinuxFunctionAppSiteConfigAppServiceLogs>>,
    /// The Connection String for linking the Linux Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsConnectionString")]
    pub r#application_insights_connection_string: Option<String>,
    /// The Instrumentation Key for connecting the Linux Function App to Application Insights.
    #[builder(into)]
    #[serde(rename = "applicationInsightsKey")]
    pub r#application_insights_key: Option<String>,
    /// An `application_stack` block as defined above.
    /// 
    /// > **Note:** If this is set, there must not be an application setting `FUNCTIONS_WORKER_RUNTIME`.
    #[builder(into)]
    #[serde(rename = "applicationStack")]
    pub r#application_stack: Option<Box<super::super::types::appservice::LinuxFunctionAppSiteConfigApplicationStack>>,
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
    pub r#cors: Option<Box<super::super::types::appservice::LinuxFunctionAppSiteConfigCors>>,
    /// Specifies a list of Default Documents for the Linux Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Option<Vec<String>>,
    /// Is detailed error logging enabled
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: Option<bool>,
    /// The number of minimum instances for this Linux Function App. Only affects apps on Elastic Premium plans.
    #[builder(into)]
    #[serde(rename = "elasticInstanceMinimum")]
    pub r#elastic_instance_minimum: Option<i32>,
    /// State of FTP / FTPS service for this function app. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer. Possible values are between `2` and `10`. Only valid in conjunction with `health_check_path`.
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
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::LinuxFunctionAppSiteConfigIpRestriction>>,
    /// The Linux FX Version
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Option<String>,
    /// The Site load balancing mode. Possible values include: `WeightedRoundRobin`, `LeastRequests`, `LeastResponseTime`, `WeightedTotalTraffic`, `RequestHash`, `PerSiteRoundRobin`. Defaults to `LeastRequests` if omitted.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: Option<String>,
    /// Managed pipeline mode. Possible values include: `Integrated`, `Classic`. Defaults to `Integrated`.
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
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::LinuxFunctionAppSiteConfigScmIpRestriction>>,
    /// Configures the minimum version of TLS required for SSL requests to the SCM site Possible values include: `1.0`, `1.1`, and `1.2`. Defaults to `1.2`.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: Option<String>,
    /// The SCM Type in use by the Linux Function App.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Option<String>,
    /// Should the Linux Function App `ip_restriction` configuration be used for the SCM also.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the Linux Web App use a 32-bit worker process. Defaults to `false`.
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
    /// The number of Workers for this Linux Function App.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxFunctionAppSiteConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "always_on".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#always_on,
                )
                .await,
            );
            map.insert(
                "api_definition_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_definition_url,
                )
                .await,
            );
            map.insert(
                "api_management_api_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_management_api_id,
                )
                .await,
            );
            map.insert(
                "app_command_line".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_command_line,
                )
                .await,
            );
            map.insert(
                "app_scale_limit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_scale_limit,
                )
                .await,
            );
            map.insert(
                "app_service_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_service_logs,
                )
                .await,
            );
            map.insert(
                "application_insights_connection_string".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_insights_connection_string,
                )
                .await,
            );
            map.insert(
                "application_insights_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_insights_key,
                )
                .await,
            );
            map.insert(
                "application_stack".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#application_stack,
                )
                .await,
            );
            map.insert(
                "container_registry_managed_identity_client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_registry_managed_identity_client_id,
                )
                .await,
            );
            map.insert(
                "container_registry_use_managed_identity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_registry_use_managed_identity,
                )
                .await,
            );
            map.insert(
                "cors".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cors,
                )
                .await,
            );
            map.insert(
                "default_documents".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_documents,
                )
                .await,
            );
            map.insert(
                "detailed_error_logging_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detailed_error_logging_enabled,
                )
                .await,
            );
            map.insert(
                "elastic_instance_minimum".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#elastic_instance_minimum,
                )
                .await,
            );
            map.insert(
                "ftps_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ftps_state,
                )
                .await,
            );
            map.insert(
                "health_check_eviction_time_in_min".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#health_check_eviction_time_in_min,
                )
                .await,
            );
            map.insert(
                "health_check_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#health_check_path,
                )
                .await,
            );
            map.insert(
                "http_2_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_2_enabled,
                )
                .await,
            );
            map.insert(
                "ip_restriction_default_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_restriction_default_action,
                )
                .await,
            );
            map.insert(
                "ip_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_restrictions,
                )
                .await,
            );
            map.insert(
                "linux_fx_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#linux_fx_version,
                )
                .await,
            );
            map.insert(
                "load_balancing_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_balancing_mode,
                )
                .await,
            );
            map.insert(
                "managed_pipeline_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#managed_pipeline_mode,
                )
                .await,
            );
            map.insert(
                "minimum_tls_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimum_tls_version,
                )
                .await,
            );
            map.insert(
                "pre_warmed_instance_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pre_warmed_instance_count,
                )
                .await,
            );
            map.insert(
                "remote_debugging_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_debugging_enabled,
                )
                .await,
            );
            map.insert(
                "remote_debugging_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote_debugging_version,
                )
                .await,
            );
            map.insert(
                "runtime_scale_monitoring_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#runtime_scale_monitoring_enabled,
                )
                .await,
            );
            map.insert(
                "scm_ip_restriction_default_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_ip_restriction_default_action,
                )
                .await,
            );
            map.insert(
                "scm_ip_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_ip_restrictions,
                )
                .await,
            );
            map.insert(
                "scm_minimum_tls_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_minimum_tls_version,
                )
                .await,
            );
            map.insert(
                "scm_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_type,
                )
                .await,
            );
            map.insert(
                "scm_use_main_ip_restriction".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_use_main_ip_restriction,
                )
                .await,
            );
            map.insert(
                "use_32_bit_worker".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_32_bit_worker,
                )
                .await,
            );
            map.insert(
                "vnet_route_all_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vnet_route_all_enabled,
                )
                .await,
            );
            map.insert(
                "websockets_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#websockets_enabled,
                )
                .await,
            );
            map.insert(
                "worker_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#worker_count,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxFunctionAppSiteConfig {
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
                    r#container_registry_managed_identity_client_id: {
                        let field_value = match fields_map.get("container_registry_managed_identity_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_registry_managed_identity_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_registry_use_managed_identity: {
                        let field_value = match fields_map.get("container_registry_use_managed_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_registry_use_managed_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#linux_fx_version: {
                        let field_value = match fields_map.get("linux_fx_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'linux_fx_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
