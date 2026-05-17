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
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "acr_use_managed_identity_credentials".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acr_use_managed_identity_credentials,
                )
                .await,
            );
            map.insert(
                "acr_user_managed_identity_client_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#acr_user_managed_identity_client_id,
                )
                .await,
            );
            map.insert(
                "always_on".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#always_on,
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
                "dotnet_framework_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dotnet_framework_version,
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
                "ip_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_restrictions,
                )
                .await,
            );
            map.insert(
                "java_container".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_container,
                )
                .await,
            );
            map.insert(
                "java_container_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_container_version,
                )
                .await,
            );
            map.insert(
                "java_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_version,
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
                "local_mysql_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_mysql_enabled,
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
                "min_tls_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_tls_version,
                )
                .await,
            );
            map.insert(
                "number_of_workers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#number_of_workers,
                )
                .await,
            );
            map.insert(
                "php_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#php_version,
                )
                .await,
            );
            map.insert(
                "python_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python_version,
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
                "scm_ip_restrictions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scm_ip_restrictions,
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
                "use_32_bit_worker_process".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_32_bit_worker_process,
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
                "windows_fx_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#windows_fx_version,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
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
                        let field_value = match fields_map.get("acr_use_managed_identity_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'acr_use_managed_identity_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#acr_user_managed_identity_client_id: {
                        let field_value = match fields_map.get("acr_user_managed_identity_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'acr_user_managed_identity_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#always_on: {
                        let field_value = match fields_map.get("always_on") {
                            Some(value) => value,
                            None => bail!("Missing field 'always_on' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#dotnet_framework_version: {
                        let field_value = match fields_map.get("dotnet_framework_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnet_framework_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#ip_restrictions: {
                        let field_value = match fields_map.get("ip_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_container: {
                        let field_value = match fields_map.get("java_container") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_container' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_container_version: {
                        let field_value = match fields_map.get("java_container_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_container_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_version: {
                        let field_value = match fields_map.get("java_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#local_mysql_enabled: {
                        let field_value = match fields_map.get("local_mysql_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_mysql_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#min_tls_version: {
                        let field_value = match fields_map.get("min_tls_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_tls_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#number_of_workers: {
                        let field_value = match fields_map.get("number_of_workers") {
                            Some(value) => value,
                            None => bail!("Missing field 'number_of_workers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#php_version: {
                        let field_value = match fields_map.get("php_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'php_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#python_version: {
                        let field_value = match fields_map.get("python_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#scm_ip_restrictions: {
                        let field_value = match fields_map.get("scm_ip_restrictions") {
                            Some(value) => value,
                            None => bail!("Missing field 'scm_ip_restrictions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#use_32_bit_worker_process: {
                        let field_value = match fields_map.get("use_32_bit_worker_process") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_32_bit_worker_process' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
