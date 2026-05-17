#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetWindowsWebAppSiteConfigApplicationStack {
    /// The Current Stack value of the Windows Web App.
    #[builder(into)]
    #[serde(rename = "currentStack")]
    pub r#current_stack: String,
    /// The docker image, including tag, used by this Windows Web App.
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
    #[builder(into)]
    #[serde(rename = "dotnetCoreVersion")]
    pub r#dotnet_core_version: String,
    /// The version of .NET in use.
    #[builder(into)]
    #[serde(rename = "dotnetVersion")]
    pub r#dotnet_version: String,
    /// The Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainer")]
    pub r#java_container: String,
    /// The Version of the Java Container in use.
    #[builder(into)]
    #[serde(rename = "javaContainerVersion")]
    pub r#java_container_version: String,
    #[builder(into)]
    #[serde(rename = "javaEmbeddedServerEnabled")]
    pub r#java_embedded_server_enabled: bool,
    /// The Version of Java in use.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: String,
    /// The Version of Node in use.
    #[builder(into)]
    #[serde(rename = "nodeVersion")]
    pub r#node_version: String,
    /// The Version of the PHP in use.
    #[builder(into)]
    #[serde(rename = "phpVersion")]
    pub r#php_version: String,
    #[builder(into)]
    #[serde(rename = "python")]
    pub r#python: bool,
    /// The Version of Python in use.
    #[builder(into)]
    #[serde(rename = "pythonVersion")]
    pub r#python_version: String,
    #[builder(into)]
    #[serde(rename = "tomcatVersion")]
    pub r#tomcat_version: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetWindowsWebAppSiteConfigApplicationStack {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "current_stack",
                    &self.r#current_stack,
                ),
                to_pulumi_object_field(
                    "docker_image_name",
                    &self.r#docker_image_name,
                ),
                to_pulumi_object_field(
                    "docker_registry_password",
                    &self.r#docker_registry_password,
                ),
                to_pulumi_object_field(
                    "docker_registry_url",
                    &self.r#docker_registry_url,
                ),
                to_pulumi_object_field(
                    "docker_registry_username",
                    &self.r#docker_registry_username,
                ),
                to_pulumi_object_field(
                    "dotnet_core_version",
                    &self.r#dotnet_core_version,
                ),
                to_pulumi_object_field(
                    "dotnet_version",
                    &self.r#dotnet_version,
                ),
                to_pulumi_object_field(
                    "java_container",
                    &self.r#java_container,
                ),
                to_pulumi_object_field(
                    "java_container_version",
                    &self.r#java_container_version,
                ),
                to_pulumi_object_field(
                    "java_embedded_server_enabled",
                    &self.r#java_embedded_server_enabled,
                ),
                to_pulumi_object_field(
                    "java_version",
                    &self.r#java_version,
                ),
                to_pulumi_object_field(
                    "node_version",
                    &self.r#node_version,
                ),
                to_pulumi_object_field(
                    "php_version",
                    &self.r#php_version,
                ),
                to_pulumi_object_field(
                    "python",
                    &self.r#python,
                ),
                to_pulumi_object_field(
                    "python_version",
                    &self.r#python_version,
                ),
                to_pulumi_object_field(
                    "tomcat_version",
                    &self.r#tomcat_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetWindowsWebAppSiteConfigApplicationStack {
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
                    r#python_version: {
                        let field_value = match fields_map.get("python_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'python_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
