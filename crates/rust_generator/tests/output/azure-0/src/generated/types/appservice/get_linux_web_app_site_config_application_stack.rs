#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLinuxWebAppSiteConfigApplicationStack {
    /// The docker image, including tag, used by this Linux Web App.
    #[builder(into)]
    #[serde(rename = "dockerImageName")]
    pub r#docker_image_name: String,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    #[serde(rename = "dockerRegistryPassword")]
    pub r#docker_registry_password: String,
    /// The URL of the container registry where the `docker_image_name` is located.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUrl")]
    pub r#docker_registry_url: String,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    #[serde(rename = "dockerRegistryUsername")]
    pub r#docker_registry_username: String,
    /// The version of .NET in use.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: String,
    #[builder(into)]
    #[serde(rename = "goVersion")]
    pub r#go_version: String,
    /// The Java server type.
    #[builder(into)]
    #[serde(rename = "javaServer")]
    pub r#java_server: String,
    /// The Version of the `java_server` in use.
    #[builder(into)]
    #[serde(rename = "javaServerVersion")]
    pub r#java_server_version: String,
    /// The Version of Java in use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: String,
    /// The version of Node in use.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: String,
    /// The version of PHP in use.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: String,
    /// The version of Python in use.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: String,
    /// The version of Ruby in use.
    #[builder(into)]
    #[serde(rename = "rubyVersion")]
    pub r#ruby_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLinuxWebAppSiteConfigApplicationStack {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLinuxWebAppSiteConfigApplicationStack {
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
