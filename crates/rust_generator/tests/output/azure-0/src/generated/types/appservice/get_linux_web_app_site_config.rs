#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppSiteConfig {
    /// Is this Linux Web App is Always On enabled.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: bool,
    /// The ID of the APIM configuration for this Linux Web App.
    #[builder(into)]
    #[serde(rename = "apiDefinitionUrl")]
    pub r#api_definition_url: String,
    /// The ID of the API Management API for this Linux Web App.
    #[builder(into)]
    #[serde(rename = "apiManagementApiId")]
    pub r#api_management_api_id: String,
    /// The command line used to launch this app.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: String,
    /// A `application_stack` block as defined above.
    #[builder(into)]
    #[serde(rename = "applicationStacks")]
    pub r#application_stacks: Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigApplicationStack>,
    /// A `auto_heal_setting` block as defined above.
    #[builder(into)]
    #[serde(rename = "autoHealSettings")]
    pub r#auto_heal_settings: Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSetting>,
    /// The Client ID of the Managed Service Identity used for connections to the Azure Container Registry.
    #[builder(into)]
    #[serde(rename = "containerRegistryManagedIdentityClientId")]
    pub r#container_registry_managed_identity_client_id: String,
    /// Do connections for Azure Container Registry use Managed Identity.
    #[builder(into)]
    #[serde(rename = "containerRegistryUseManagedIdentity")]
    pub r#container_registry_use_managed_identity: bool,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigCor>,
    /// The list of Default Documents for the Linux Web App.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Vec<String>,
    /// Is Detailed Error Logging enabled.
    #[builder(into)]
    #[serde(rename = "detailedErrorLoggingEnabled")]
    pub r#detailed_error_logging_enabled: bool,
    /// The State of FTP / FTPS service.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: String,
    /// The amount of time in minutes that a node can be unhealthy before being removed from the load balancer.
    #[builder(into)]
    #[serde(rename = "healthCheckEvictionTimeInMin")]
    pub r#health_check_eviction_time_in_min: i32,
    /// The path to the Health Check endpoint.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: String,
    /// Is HTTP2.0 enabled.
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: bool,
    /// The Default action for traffic that does not match any `ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "ipRestrictionDefaultAction")]
    pub r#ip_restriction_default_action: String,
    /// A `ip_restriction` block as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigIpRestriction>,
    /// The `LinuxFXVersion` string.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: String,
    /// The site Load Balancing Mode.
    #[builder(into)]
    #[serde(rename = "loadBalancingMode")]
    pub r#load_balancing_mode: String,
    /// Is the Local MySQL enabled.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: bool,
    /// The Managed Pipeline Mode.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: String,
    /// The Minimum version of TLS for requests.
    #[builder(into)]
    #[serde(rename = "minimumTlsVersion")]
    pub r#minimum_tls_version: String,
    /// Is Remote Debugging enabled.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: bool,
    /// The Remote Debugging Version.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: String,
    /// The Default action for traffic that does not match any `scm_ip_restriction` rule.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictionDefaultAction")]
    pub r#scm_ip_restriction_default_action: String,
    /// A `scm_ip_restriction` block as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigScmIpRestriction>,
    /// The Minimum version of TLS for requests to SCM.
    #[builder(into)]
    #[serde(rename = "scmMinimumTlsVersion")]
    pub r#scm_minimum_tls_version: String,
    /// The Source Control Management Type in use.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: String,
    /// Is the Linux Web App `ip_restriction` configuration used for the SCM also.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: bool,
    /// Does the Linux Web App use a 32-bit worker.
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
    /// The number of Workers for this Linux App Service.
    #[builder(into)]
    #[serde(rename = "workerCount")]
    pub r#worker_count: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxWebAppSiteConfig {
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
                    "alwaysOn",
                    &self.r#always_on,
                ),
                to_pulumi_object_field(
                    "apiDefinitionUrl",
                    &self.r#api_definition_url,
                ),
                to_pulumi_object_field(
                    "apiManagementApiId",
                    &self.r#api_management_api_id,
                ),
                to_pulumi_object_field(
                    "appCommandLine",
                    &self.r#app_command_line,
                ),
                to_pulumi_object_field(
                    "applicationStacks",
                    &self.r#application_stacks,
                ),
                to_pulumi_object_field(
                    "autoHealSettings",
                    &self.r#auto_heal_settings,
                ),
                to_pulumi_object_field(
                    "containerRegistryManagedIdentityClientId",
                    &self.r#container_registry_managed_identity_client_id,
                ),
                to_pulumi_object_field(
                    "containerRegistryUseManagedIdentity",
                    &self.r#container_registry_use_managed_identity,
                ),
                to_pulumi_object_field(
                    "cors",
                    &self.r#cors,
                ),
                to_pulumi_object_field(
                    "defaultDocuments",
                    &self.r#default_documents,
                ),
                to_pulumi_object_field(
                    "detailedErrorLoggingEnabled",
                    &self.r#detailed_error_logging_enabled,
                ),
                to_pulumi_object_field(
                    "ftpsState",
                    &self.r#ftps_state,
                ),
                to_pulumi_object_field(
                    "healthCheckEvictionTimeInMin",
                    &self.r#health_check_eviction_time_in_min,
                ),
                to_pulumi_object_field(
                    "healthCheckPath",
                    &self.r#health_check_path,
                ),
                to_pulumi_object_field(
                    "http2Enabled",
                    &self.r#http_2_enabled,
                ),
                to_pulumi_object_field(
                    "ipRestrictionDefaultAction",
                    &self.r#ip_restriction_default_action,
                ),
                to_pulumi_object_field(
                    "ipRestrictions",
                    &self.r#ip_restrictions,
                ),
                to_pulumi_object_field(
                    "linuxFxVersion",
                    &self.r#linux_fx_version,
                ),
                to_pulumi_object_field(
                    "loadBalancingMode",
                    &self.r#load_balancing_mode,
                ),
                to_pulumi_object_field(
                    "localMysqlEnabled",
                    &self.r#local_mysql_enabled,
                ),
                to_pulumi_object_field(
                    "managedPipelineMode",
                    &self.r#managed_pipeline_mode,
                ),
                to_pulumi_object_field(
                    "minimumTlsVersion",
                    &self.r#minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "remoteDebuggingEnabled",
                    &self.r#remote_debugging_enabled,
                ),
                to_pulumi_object_field(
                    "remoteDebuggingVersion",
                    &self.r#remote_debugging_version,
                ),
                to_pulumi_object_field(
                    "scmIpRestrictionDefaultAction",
                    &self.r#scm_ip_restriction_default_action,
                ),
                to_pulumi_object_field(
                    "scmIpRestrictions",
                    &self.r#scm_ip_restrictions,
                ),
                to_pulumi_object_field(
                    "scmMinimumTlsVersion",
                    &self.r#scm_minimum_tls_version,
                ),
                to_pulumi_object_field(
                    "scmType",
                    &self.r#scm_type,
                ),
                to_pulumi_object_field(
                    "scmUseMainIpRestriction",
                    &self.r#scm_use_main_ip_restriction,
                ),
                to_pulumi_object_field(
                    "use32BitWorker",
                    &self.r#use_32_bit_worker,
                ),
                to_pulumi_object_field(
                    "vnetRouteAllEnabled",
                    &self.r#vnet_route_all_enabled,
                ),
                to_pulumi_object_field(
                    "websocketsEnabled",
                    &self.r#websockets_enabled,
                ),
                to_pulumi_object_field(
                    "workerCount",
                    &self.r#worker_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxWebAppSiteConfig {
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
                        let field_value = match fields_map.get("alwaysOn") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysOn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_definition_url: {
                        let field_value = match fields_map.get("apiDefinitionUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiDefinitionUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#api_management_api_id: {
                        let field_value = match fields_map.get("apiManagementApiId") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiManagementApiId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_command_line: {
                        let field_value = match fields_map.get("appCommandLine") {
                            Some(value) => value,
                            None => bail!("Missing field 'appCommandLine' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_stacks: {
                        let field_value = match fields_map.get("applicationStacks") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationStacks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_heal_settings: {
                        let field_value = match fields_map.get("autoHealSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoHealSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_registry_managed_identity_client_id: {
                        let field_value = match fields_map.get("containerRegistryManagedIdentityClientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerRegistryManagedIdentityClientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_registry_use_managed_identity: {
                        let field_value = match fields_map.get("containerRegistryUseManagedIdentity") {
                            Some(value) => value,
                            None => bail!("Missing field 'containerRegistryUseManagedIdentity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("defaultDocuments") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultDocuments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#detailed_error_logging_enabled: {
                        let field_value = match fields_map.get("detailedErrorLoggingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'detailedErrorLoggingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ftps_state: {
                        let field_value = match fields_map.get("ftpsState") {
                            Some(value) => value,
                            None => bail!("Missing field 'ftpsState' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_eviction_time_in_min: {
                        let field_value = match fields_map.get("healthCheckEvictionTimeInMin") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckEvictionTimeInMin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_path: {
                        let field_value = match fields_map.get("healthCheckPath") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckPath' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_enabled: {
                        let field_value = match fields_map.get("http2Enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'http2Enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_restriction_default_action: {
                        let field_value = match fields_map.get("ipRestrictionDefaultAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipRestrictionDefaultAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_restrictions: {
                        let field_value = match fields_map.get("ipRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linux_fx_version: {
                        let field_value = match fields_map.get("linuxFxVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'linuxFxVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_balancing_mode: {
                        let field_value = match fields_map.get("loadBalancingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'loadBalancingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_mysql_enabled: {
                        let field_value = match fields_map.get("localMysqlEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'localMysqlEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_pipeline_mode: {
                        let field_value = match fields_map.get("managedPipelineMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedPipelineMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_tls_version: {
                        let field_value = match fields_map.get("minimumTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimumTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_debugging_enabled: {
                        let field_value = match fields_map.get("remoteDebuggingEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'remoteDebuggingEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_debugging_version: {
                        let field_value = match fields_map.get("remoteDebuggingVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'remoteDebuggingVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_ip_restriction_default_action: {
                        let field_value = match fields_map.get("scmIpRestrictionDefaultAction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmIpRestrictionDefaultAction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_ip_restrictions: {
                        let field_value = match fields_map.get("scmIpRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmIpRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_minimum_tls_version: {
                        let field_value = match fields_map.get("scmMinimumTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmMinimumTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_type: {
                        let field_value = match fields_map.get("scmType") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scm_use_main_ip_restriction: {
                        let field_value = match fields_map.get("scmUseMainIpRestriction") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmUseMainIpRestriction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_32_bit_worker: {
                        let field_value = match fields_map.get("use32BitWorker") {
                            Some(value) => value,
                            None => bail!("Missing field 'use32BitWorker' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vnet_route_all_enabled: {
                        let field_value = match fields_map.get("vnetRouteAllEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'vnetRouteAllEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#websockets_enabled: {
                        let field_value = match fields_map.get("websocketsEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'websocketsEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#worker_count: {
                        let field_value = match fields_map.get("workerCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'workerCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
