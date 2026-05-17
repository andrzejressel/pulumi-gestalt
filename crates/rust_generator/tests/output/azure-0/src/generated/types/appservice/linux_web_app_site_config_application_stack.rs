#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSiteConfigApplicationStack {
    /// The docker image, including tag, to be used. e.g. `appsvc/staticsite:latest`.
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
    /// The version of .NET to use. Possible values include `3.1`, `5.0`, `6.0`, `7.0`, `8.0` and `9.0`.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: Option<String>,
    /// The version of Go to use. Possible values include `1.18`, and `1.19`.
    #[builder(into)]
    #[serde(rename = "goVersion")]
    pub r#go_version: Option<String>,
    /// The Java server type. Possible values include `JAVA`, `TOMCAT`, and `JBOSSEAP`.
    /// 
    /// > **NOTE:** `JBOSSEAP` requires a Premium Service Plan SKU to be a valid option.
    #[builder(into)]
    #[serde(rename = "javaServer")]
    pub r#java_server: Option<String>,
    /// The Version of the `java_server` to use.
    #[builder(into)]
    #[serde(rename = "javaServerVersion")]
    pub r#java_server_version: Option<String>,
    /// The Version of Java to use. Possible values include `8`, `11`, `17`, and `21`.
    /// 
    /// > **NOTE:** The valid version combinations for `java_version`, `java_server` and `java_server_version` can be checked from the command line via `az webapp list-runtimes --linux`.
    /// 
    /// > **NOTE:** `java_server`, `java_server_version`, and `java_version` must all be specified if building a java app
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// The version of Node to run. Possible values include `12-lts`, `14-lts`, `16-lts`, `18-lts` and `20-lts`. This property conflicts with `java_version`.
    /// 
    /// > **NOTE:** 10.x versions have been/are being deprecated so may cease to work for new resources in the future and may be removed from the provider.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: Option<String>,
    /// The version of PHP to run. Possible values are `7.4`, `8.0`, `8.1`, `8.2` and `8.3`.
    /// 
    /// > **NOTE:** version `7.4` is deprecated and will be removed from the provider in a future version.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: Option<String>,
    /// The version of Python to run. Possible values include `3.7`, `3.8`, `3.9`, `3.10`, `3.11` and `3.12`.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: Option<String>,
    /// The version of Ruby to run. Possible values include `2.6` and `2.7`.
    #[builder(into)]
    #[serde(rename = "rubyVersion")]
    pub r#ruby_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LinuxWebAppSiteConfigApplicationStack {
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
                "dotnet_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#dotnet_version,
                )
                .await,
            );
            map.insert(
                "go_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#go_version,
                )
                .await,
            );
            map.insert(
                "java_server".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_server,
                )
                .await,
            );
            map.insert(
                "java_server_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#java_server_version,
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
                "python_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#python_version,
                )
                .await,
            );
            map.insert(
                "ruby_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ruby_version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LinuxWebAppSiteConfigApplicationStack {
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
                    r#dotnet_version: {
                        let field_value = match fields_map.get("dotnet_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnet_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#go_version: {
                        let field_value = match fields_map.get("go_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'go_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_server: {
                        let field_value = match fields_map.get("java_server") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_server' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_server_version: {
                        let field_value = match fields_map.get("java_server_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'java_server_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#python_version: {
                        let field_value = match fields_map.get("python_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ruby_version: {
                        let field_value = match fields_map.get("ruby_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'ruby_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
