#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppServiceSiteConfig {
    /// Are Managed Identity Credentials used for Azure Container Registry pull.
    #[builder(into)]
    #[serde(rename = "acrUseManagedIdentityCredentials")]
    pub r#acr_use_managed_identity_credentials: bool,
    /// The User Managed Identity Client Id.
    #[builder(into)]
    #[serde(rename = "acrUserManagedIdentityClientId")]
    pub r#acr_user_managed_identity_client_id: String,
    /// Is the app loaded at all times?
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: bool,
    /// App command line to launch.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: String,
    /// A `cors` block as defined above.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Vec<super::super::types::appservice::GetAppServiceSiteConfigCor>,
    /// The ordering of default documents to load, if an address isn't specified.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Vec<String>,
    /// The version of the .NET framework's CLR used in this App Service.
    #[builder(into)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: String,
    /// State of FTP / FTPS service for this AppService.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: String,
    /// The health check path to be pinged by App Service.
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: String,
    /// Is HTTP2 Enabled on this App Service?
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: bool,
    /// One or more `ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Vec<super::super::types::appservice::GetAppServiceSiteConfigIpRestriction>,
    /// The Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: String,
    /// The version of the Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: String,
    /// The version of Java in use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: String,
    /// Linux App Framework and version for the AppService.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: String,
    /// Is "MySQL In App" Enabled? This runs a local MySQL instance with your app and shares resources from the App Service plan.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: bool,
    /// The Managed Pipeline Mode used in this App Service.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: String,
    /// The minimum supported TLS version for this App Service.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: String,
    /// The scaled number of workers (for per site scaling) of this App Service.
    #[builder(into)]
    #[serde(rename = "numberOfWorkers")]
    pub r#number_of_workers: i32,
    /// The version of PHP used in this App Service.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: String,
    /// The version of Python used in this App Service.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: String,
    /// Is Remote Debugging Enabled in this App Service?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: bool,
    /// Which version of Visual Studio is the Remote Debugger compatible with?
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: String,
    /// One or more `scm_ip_restriction` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Vec<super::super::types::appservice::GetAppServiceSiteConfigScmIpRestriction>,
    /// The type of Source Control enabled for this App Service.
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: String,
    /// IP security restrictions for scm to use main.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: bool,
    /// Does the App Service run in 32 bit mode, rather than 64 bit mode?
    #[builder(into)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: bool,
    /// (Optional) Should all outbound traffic to have Virtual Network Security Groups and User Defined Routes applied?
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: bool,
    /// Are WebSockets enabled for this App Service?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: bool,
    /// Windows Container Docker Image for the AppService.
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAppServiceSiteConfig {
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
                    "acrUseManagedIdentityCredentials",
                    &self.r#acr_use_managed_identity_credentials,
                ),
                to_pulumi_object_field(
                    "acrUserManagedIdentityClientId",
                    &self.r#acr_user_managed_identity_client_id,
                ),
                to_pulumi_object_field(
                    "alwaysOn",
                    &self.r#always_on,
                ),
                to_pulumi_object_field(
                    "appCommandLine",
                    &self.r#app_command_line,
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
                    "dotnetFrameworkVersion",
                    &self.r#dotnet_framework_version,
                ),
                to_pulumi_object_field(
                    "ftpsState",
                    &self.r#ftps_state,
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
                    "ipRestrictions",
                    &self.r#ip_restrictions,
                ),
                to_pulumi_object_field(
                    "javaContainer",
                    &self.r#java_container,
                ),
                to_pulumi_object_field(
                    "javaContainerVersion",
                    &self.r#java_container_version,
                ),
                to_pulumi_object_field(
                    "javaVersion",
                    &self.r#java_version,
                ),
                to_pulumi_object_field(
                    "linuxFxVersion",
                    &self.r#linux_fx_version,
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
                    "minTlsVersion",
                    &self.r#min_tls_version,
                ),
                to_pulumi_object_field(
                    "numberOfWorkers",
                    &self.r#number_of_workers,
                ),
                to_pulumi_object_field(
                    "phpVersion",
                    &self.r#php_version,
                ),
                to_pulumi_object_field(
                    "pythonVersion",
                    &self.r#python_version,
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
                    "scmIpRestrictions",
                    &self.r#scm_ip_restrictions,
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
                    "use32BitWorkerProcess",
                    &self.r#use_32_bit_worker_process,
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
                    "windowsFxVersion",
                    &self.r#windows_fx_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAppServiceSiteConfig {
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
                    r#acr_use_managed_identity_credentials: {
                        let field_value = match fields_map.get("acrUseManagedIdentityCredentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'acrUseManagedIdentityCredentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#acr_user_managed_identity_client_id: {
                        let field_value = match fields_map.get("acrUserManagedIdentityClientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'acrUserManagedIdentityClientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#always_on: {
                        let field_value = match fields_map.get("alwaysOn") {
                            Some(value) => value,
                            None => bail!("Missing field 'alwaysOn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#dotnet_framework_version: {
                        let field_value = match fields_map.get("dotnetFrameworkVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetFrameworkVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ip_restrictions: {
                        let field_value = match fields_map.get("ipRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_container: {
                        let field_value = match fields_map.get("javaContainer") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaContainer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_container_version: {
                        let field_value = match fields_map.get("javaContainerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaContainerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_version: {
                        let field_value = match fields_map.get("javaVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#min_tls_version: {
                        let field_value = match fields_map.get("minTlsVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'minTlsVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_workers: {
                        let field_value = match fields_map.get("numberOfWorkers") {
                            Some(value) => value,
                            None => bail!("Missing field 'numberOfWorkers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#php_version: {
                        let field_value = match fields_map.get("phpVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'phpVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_version: {
                        let field_value = match fields_map.get("pythonVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'pythonVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scm_ip_restrictions: {
                        let field_value = match fields_map.get("scmIpRestrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scmIpRestrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#use_32_bit_worker_process: {
                        let field_value = match fields_map.get("use32BitWorkerProcess") {
                            Some(value) => value,
                            None => bail!("Missing field 'use32BitWorkerProcess' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#windows_fx_version: {
                        let field_value = match fields_map.get("windowsFxVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'windowsFxVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
