#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSiteConfigApplicationStack {
    /// The Application Stack for the Windows Web App. Possible values include `dotnet`, `dotnetcore`, `node`, `python`, `php`, and `java`.
    /// 
    /// > **NOTE:** Whilst this property is Optional omitting it can cause unexpected behaviour, in particular for display of settings in the Azure Portal.
    /// 
    /// > **NOTE:** Windows Web apps can configure multiple `app_stack` properties, it is recommended to always configure this `Optional` value and set it to the primary application stack of your app to ensure correct operation of this resource and display the correct metadata in the Azure Portal.
    #[builder(into)]
    #[serde(rename = "currentStack")]
    pub r#current_stack: Option<String>,
    /// The docker image, including tag, to be used. e.g. `azure-app-service/windows/parkingpage:latest`.
    #[builder(into)]
    #[serde(rename = "dockerImageName")]
    pub r#docker_image_name: Option<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    /// 
    /// > **NOTE:** `docker_registry_url`, `docker_registry_username`, and `docker_registry_password` replace the use of the `app_settings` values of `DOCKER_REGISTRY_SERVER_URL`, `DOCKER_REGISTRY_SERVER_USERNAME` and `DOCKER_REGISTRY_SERVER_PASSWORD` respectively, these values will be managed by the provider and should not be specified in the `app_settings` map.
    #[builder(into)]
    #[serde(rename = "dockerRegistryPassword")]
    pub r#docker_registry_password: Option<String>,
    /// The URL of the container registry where the `docker_image_name` is located. e.g. `https://index.docker.io` or `https://mcr.microsoft.com`. This value is required with `docker_image_name`.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUrl")]
    pub r#docker_registry_url: Option<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUsername")]
    pub r#docker_registry_username: Option<String>,
    /// The version of .NET to use when `current_stack` is set to `dotnetcore`. Possible values include `v4.0`.
    #[builder(into)]
    #[serde(rename = "dotnetCoreVersion")]
    pub r#dotnet_core_version: Option<String>,
    /// The version of .NET to use when `current_stack` is set to `dotnet`. Possible values include `v2.0`,`v3.0`, `v4.0`, `v5.0`, `v6.0`, `v7.0`, `v8.0` and `v9.0`.
    /// 
    /// > **NOTE:** The Portal displayed values and the actual underlying API values differ for this setting, as follows:
    /// Portal Value | API value
    /// :--|--:
    /// ASP.NET V3.5 | v2.0
    /// ASP.NET V4.8 | v4.0
    /// .NET 6 (LTS) | v6.0
    /// .NET 7 (STS) | v7.0
    /// .NET 8 (LTS) | v8.0
    /// .NET 9 (STS) | v9.0
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Option<String>,
    #[builder(into)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: Option<String>,
    #[builder(into)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: Option<String>,
    /// Should the Java Embedded Server (Java SE) be used to run the app.
    #[builder(into)]
    #[serde(rename = "javaEmbeddedServerEnabled")]
    pub r#java_embedded_server_enabled: Option<bool>,
    /// The version of Java to use when `current_stack` is set to `java`. 
    /// 
    /// > **NOTE:** For currently supported versions, please see the official documentation. Some example values include: `1.8`, `1.8.0_322`,  `11`, `11.0.14`, `17` and `17.0.2`
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// The version of node to use when `current_stack` is set to `node`. Possible values are `~12`, `~14`, `~16`, `~18` and `~20`.
    /// 
    /// > **NOTE:** This property conflicts with `java_version`.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Option<String>,
    /// The version of PHP to use when `current_stack` is set to `php`. Possible values are `7.1`, `7.4` and `Off`.
    /// 
    /// > **NOTE:** The value `Off` is used to signify latest supported by the service.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Option<String>,
    /// Specifies whether this is a Python app. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "python")]
    pub r#python: Option<bool>,
    /// The version of Tomcat the Java App should use. Conflicts with `java_embedded_server_enabled`
    /// 
    /// > **NOTE:** See the official documentation for current supported versions.  Some example valuess include: `10.0`, `10.0.20`.
    #[builder(into)]
    #[serde(rename = "tomcatVersion")]
    pub r#tomcat_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSiteConfigApplicationStack {
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
                "current_stack".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_stack,
                )
                .await,
            );
            map.insert(
                "docker_image_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_image_name,
                )
                .await,
            );
            map.insert(
                "docker_registry_password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_registry_password,
                )
                .await,
            );
            map.insert(
                "docker_registry_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_registry_url,
                )
                .await,
            );
            map.insert(
                "docker_registry_username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#docker_registry_username,
                )
                .await,
            );
            map.insert(
                "dotnet_core_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dotnet_core_version,
                )
                .await,
            );
            map.insert(
                "dotnet_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dotnet_version,
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
                "java_embedded_server_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_embedded_server_enabled,
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
                "node_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#node_version,
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
                "python".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python,
                )
                .await,
            );
            map.insert(
                "tomcat_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tomcat_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSiteConfigApplicationStack {
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
                    r#current_stack: {
                        let field_value = match fields_map.get("current_stack") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_stack' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_image_name: {
                        let field_value = match fields_map.get("docker_image_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_image_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_password: {
                        let field_value = match fields_map.get("docker_registry_password") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_registry_password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_url: {
                        let field_value = match fields_map.get("docker_registry_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_registry_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_username: {
                        let field_value = match fields_map.get("docker_registry_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'docker_registry_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_core_version: {
                        let field_value = match fields_map.get("dotnet_core_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnet_core_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_version: {
                        let field_value = match fields_map.get("dotnet_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnet_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#java_embedded_server_enabled: {
                        let field_value = match fields_map.get("java_embedded_server_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_embedded_server_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_version: {
                        let field_value = match fields_map.get("node_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'node_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#python: {
                        let field_value = match fields_map.get("python") {
                            Some(value) => value,
                            None => bail!("Missing field 'python' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tomcat_version: {
                        let field_value = match fields_map.get("tomcat_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'tomcat_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
