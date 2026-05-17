#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppServiceSiteConfig {
    /// Are Managed Identity Credentials used for Azure Container Registry pull
    #[builder(into)]
    #[serde(rename = "acrUseManagedIdentityCredentials")]
    pub r#acr_use_managed_identity_credentials: Option<bool>,
    /// If using User Managed Identity, the User Managed Identity Client Id
    /// 
    /// > **NOTE:** When using User Managed Identity with Azure Container Registry the Identity will need to have the [ACRPull role assigned](https://docs.microsoft.com/azure/container-registry/container-registry-authentication-managed-identity#example-1-access-with-a-user-assigned-identity)
    #[builder(into)]
    #[serde(rename = "acrUserManagedIdentityClientId")]
    pub r#acr_user_managed_identity_client_id: Option<String>,
    /// Should the app be loaded at all times? Defaults to `false`.
    /// 
    /// > **NOTE:** when using an App Service Plan in the `Free` or `Shared` Tiers `always_on` must be set to `false`.
    #[builder(into)]
    #[serde(rename = "alwaysOn")]
    pub r#always_on: Option<bool>,
    /// App command line to launch, e.g. `/sbin/myserver -b 0.0.0.0`.
    #[builder(into)]
    #[serde(rename = "appCommandLine")]
    pub r#app_command_line: Option<String>,
    /// The name of the slot to automatically swap to during deployment
    #[builder(into)]
    #[serde(rename = "autoSwapSlotName")]
    pub r#auto_swap_slot_name: Option<String>,
    /// A `cors` block as defined below.
    #[builder(into)]
    #[serde(rename = "cors")]
    pub r#cors: Option<Box<super::super::types::appservice::AppServiceSiteConfigCors>>,
    /// The ordering of default documents to load, if an address isn't specified.
    #[builder(into)]
    #[serde(rename = "defaultDocuments")]
    pub r#default_documents: Option<Vec<String>>,
    /// The version of the .NET framework's CLR used in this App Service. Possible values are `v2.0` (which will use the latest version of the .NET framework for the .NET CLR v2 - currently `.net 3.5`), `v4.0` (which corresponds to the latest version of the .NET CLR v4 - which at the time of writing is `.net 4.7.1`), `v5.0` and `v6.0`. [For more information on which .NET CLR version to use based on the .NET framework you're targeting - please see this table](https://en.wikipedia.org/wiki/.NET_Framework_version_history#Overview). Defaults to `v4.0`.
    #[builder(into)]
    #[serde(rename = "dotnetFrameworkVersion")]
    pub r#dotnet_framework_version: Option<String>,
    /// State of FTP / FTPS service for this App Service. Possible values include: `AllAllowed`, `FtpsOnly` and `Disabled`.
    #[builder(into)]
    #[serde(rename = "ftpsState")]
    pub r#ftps_state: Option<String>,
    /// The health check path to be pinged by App Service. [For more information - please see App Service health check announcement](https://azure.github.io/AppService/2020/08/24/healthcheck-on-app-service.html).
    #[builder(into)]
    #[serde(rename = "healthCheckPath")]
    pub r#health_check_path: Option<String>,
    /// Is HTTP2 Enabled on this App Service? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "http2Enabled")]
    pub r#http_2_enabled: Option<bool>,
    /// A list of objects representing ip restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into)]
    #[serde(rename = "ipRestrictions")]
    pub r#ip_restrictions: Option<Vec<super::super::types::appservice::AppServiceSiteConfigIpRestriction>>,
    /// The Java Container to use. If specified `java_version` and `java_container_version` must also be specified. Possible values are `JAVA`, `JETTY`, and `TOMCAT`.
    #[builder(into)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: Option<String>,
    /// The version of the Java Container to use. If specified `java_version` and `java_container` must also be specified.
    #[builder(into)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: Option<String>,
    /// The version of Java to use. If specified `java_container` and `java_container_version` must also be specified. Possible values are `1.7`, `1.8` and `11` and their specific versions - except for Java 11 (e.g. `1.7.0_80`, `1.8.0_181`, `11`)
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// Linux App Framework and version for the App Service. Possible options are a Docker container (`DOCKER|<user/image:tag>`), a base-64 encoded Docker Compose file (`COMPOSE|${filebase64("compose.yml")}`) or a base-64 encoded Kubernetes Manifest (`KUBE|${filebase64("kubernetes.yml")}`).
    /// 
    /// > **NOTE:** To set this property the App Service Plan to which the App belongs must be configured with `kind = "Linux"`, and `reserved = true` or the API will reject any value supplied.
    #[builder(into)]
    #[serde(rename = "linuxFxVersion")]
    pub r#linux_fx_version: Option<String>,
    /// Is "MySQL In App" Enabled? This runs a local MySQL instance with your app and shares resources from the App Service plan.
    /// 
    /// > **NOTE:** MySQL In App is not intended for production environments and will not scale beyond a single instance. Instead you may wish to use Azure Database for MySQL.
    #[builder(into)]
    #[serde(rename = "localMysqlEnabled")]
    pub r#local_mysql_enabled: Option<bool>,
    /// The Managed Pipeline Mode. Possible values are `Integrated` and `Classic`. Defaults to `Integrated`.
    #[builder(into)]
    #[serde(rename = "managedPipelineMode")]
    pub r#managed_pipeline_mode: Option<String>,
    /// The minimum supported TLS version for the app service. Possible values are `1.0`, `1.1`, and `1.2`. Defaults to `1.2` for new app services.
    #[builder(into)]
    #[serde(rename = "minTlsVersion")]
    pub r#min_tls_version: Option<String>,
    /// The scaled number of workers (for per site scaling) of this App Service. Requires that `per_site_scaling` is enabled on the `azure.appservice.Plan`. [For more information - please see Microsoft documentation on high-density hosting](https://docs.microsoft.com/azure/app-service/manage-scale-per-app).
    #[builder(into)]
    #[serde(rename = "numberOfWorkers")]
    pub r#number_of_workers: Option<i32>,
    /// The version of PHP to use in this App Service. Possible values are `5.5`, `5.6`, `7.0`, `7.1`, `7.2`, `7.3` and `7.4`.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Option<String>,
    /// The version of Python to use in this App Service. Possible values are `2.7` and `3.4`.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Option<String>,
    /// Is Remote Debugging Enabled? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingEnabled")]
    pub r#remote_debugging_enabled: Option<bool>,
    /// Which version of Visual Studio should the Remote Debugger be compatible with? Possible values are `VS2017`, `VS2019`, `VS2022`.
    #[builder(into)]
    #[serde(rename = "remoteDebuggingVersion")]
    pub r#remote_debugging_version: Option<String>,
    /// A list of `scm_ip_restriction` objects representing IP restrictions as defined below.
    /// 
    /// > **NOTE** User has to explicitly set `scm_ip_restriction` to empty slice (`[]`) to remove it.
    #[builder(into)]
    #[serde(rename = "scmIpRestrictions")]
    pub r#scm_ip_restrictions: Option<Vec<super::super::types::appservice::AppServiceSiteConfigScmIpRestriction>>,
    /// The type of Source Control enabled for this App Service. Defaults to `None`. Possible values are: `BitbucketGit`, `BitbucketHg`, `CodePlexGit`, `CodePlexHg`, `Dropbox`, `ExternalGit`, `ExternalHg`, `GitHub`, `LocalGit`, `None`, `OneDrive`, `Tfs`, `VSO`, and `VSTSRM`
    #[builder(into)]
    #[serde(rename = "scmType")]
    pub r#scm_type: Option<String>,
    /// IP security restrictions for scm to use main. Defaults to `false`. 
    /// 
    /// > **NOTE** Any `scm_ip_restriction` blocks configured are ignored by the service when `scm_use_main_ip_restriction` is set to `true`. Any scm restrictions will become active if this is subsequently set to `false` or removed.
    #[builder(into)]
    #[serde(rename = "scmUseMainIpRestriction")]
    pub r#scm_use_main_ip_restriction: Option<bool>,
    /// Should the App Service run in 32 bit mode, rather than 64 bit mode?
    /// 
    /// > **NOTE:** when using an App Service Plan in the `Free` or `Shared` Tiers `use_32_bit_worker_process` must be set to `true`.
    #[builder(into)]
    #[serde(rename = "use32BitWorkerProcess")]
    pub r#use_32_bit_worker_process: Option<bool>,
    #[builder(into)]
    #[serde(rename = "vnetRouteAllEnabled")]
    pub r#vnet_route_all_enabled: Option<bool>,
    /// Should WebSockets be enabled?
    #[builder(into)]
    #[serde(rename = "websocketsEnabled")]
    pub r#websockets_enabled: Option<bool>,
    /// The Windows Docker container image (`DOCKER|<user/image:tag>`)
    #[builder(into)]
    #[serde(rename = "windowsFxVersion")]
    pub r#windows_fx_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AppServiceSiteConfig {
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
                "auto_swap_slot_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auto_swap_slot_name,
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
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AppServiceSiteConfig {
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
                    r#auto_swap_slot_name: {
                        let field_value = match fields_map.get("auto_swap_slot_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_swap_slot_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
