#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotSiteConfigApplicationStack {
    /// The Application Stack for the Windows Web App. Possible values include `dotnet`, `dotnetcore`, `node`, `python`, `php`, and `java`.
    /// 
    /// > **NOTE:** Whilst this property is Optional omitting it can cause unexpected behaviour, in particular for display of settings in the Azure Portal.
    #[builder(into)]
    pub r#current_stack: Option<String>,
    /// The docker image, including tag, to be used. e.g. `azure-app-service/windows/parkingpage:latest`.
    #[builder(into)]
    pub r#docker_image_name: Option<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    /// 
    /// > **NOTE:** `docker_registry_url`, `docker_registry_username`, and `docker_registry_password` replace the use of the `app_settings` values of `DOCKER_REGISTRY_SERVER_URL`, `DOCKER_REGISTRY_SERVER_USERNAME` and `DOCKER_REGISTRY_SERVER_PASSWORD` respectively, these values will be managed by the provider and should not be specified in the `app_settings` map.
    #[builder(into)]
    pub r#docker_registry_password: Option<String>,
    /// The URL of the container registry where the `docker_image_name` is located. e.g. `https://index.docker.io` or `https://mcr.microsoft.com`. This value is required with `docker_image_name`.
    #[builder(into)]
    pub r#docker_registry_url: Option<String>,
    /// The User Name to use for authentication against the registry to pull the image.
    #[builder(into)]
    pub r#docker_registry_username: Option<String>,
    /// The version of .NET to use when `current_stack` is set to `dotnetcore`. Possible values include `v4.0`.
    #[builder(into)]
    pub r#dotnet_core_version: Option<String>,
    /// The version of .NET to use when `current_stack` is set to `dotnet`. Possible values include `v2.0`,`v3.0`, `v4.0`, `v5.0`, `v6.0`, `v7.0`, `v8.0` and `v9.0`.
    #[builder(into)]
    pub r#dotnet_version: Option<String>,
    #[builder(into)]
    pub r#java_container: Option<String>,
    #[builder(into)]
    pub r#java_container_version: Option<String>,
    /// Should the Java Embedded Server (Java SE) be used to run the app.
    #[builder(into)]
    pub r#java_embedded_server_enabled: Option<bool>,
    /// The version of Java to use when `current_stack` is set to `java`. Possible values include `1.7`, `1.8`, `11` and `17`. Required with `java_container` and `java_container_version`.
    /// 
    /// > **NOTE:** For compatible combinations of `java_version`, `java_container` and `java_container_version` users can use `az webapp list-runtimes` from command line.
    #[builder(into)]
    pub r#java_version: Option<String>,
    /// The version of node to use when `current_stack` is set to `node`. Possible values include `~12`, `~14`, `~16`, `~18` and `~20`.
    /// 
    /// > **NOTE:** This property conflicts with `java_version`.
    #[builder(into)]
    pub r#node_version: Option<String>,
    /// The version of PHP to use when `current_stack` is set to `php`. Possible values are `7.1`, `7.4` and `Off`.
    /// 
    /// > **NOTE:** The value `Off` is used to signify latest supported by the service.
    #[builder(into)]
    pub r#php_version: Option<String>,
    /// The app is a Python app. Defaults to `false`.
    #[builder(into)]
    pub r#python: Option<bool>,
    /// The version of Tomcat the Java App should use.
    /// 
    /// > **NOTE:** See the official documentation for current supported versions.
    #[builder(into)]
    pub r#tomcat_version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WindowsWebAppSlotSiteConfigApplicationStack {
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
                    "currentStack",
                    &self.r#current_stack,
                ),
                to_pulumi_object_field(
                    "dockerImageName",
                    &self.r#docker_image_name,
                ),
                to_pulumi_object_field(
                    "dockerRegistryPassword",
                    &self.r#docker_registry_password,
                ),
                to_pulumi_object_field(
                    "dockerRegistryUrl",
                    &self.r#docker_registry_url,
                ),
                to_pulumi_object_field(
                    "dockerRegistryUsername",
                    &self.r#docker_registry_username,
                ),
                to_pulumi_object_field(
                    "dotnetCoreVersion",
                    &self.r#dotnet_core_version,
                ),
                to_pulumi_object_field(
                    "dotnetVersion",
                    &self.r#dotnet_version,
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
                    "javaEmbeddedServerEnabled",
                    &self.r#java_embedded_server_enabled,
                ),
                to_pulumi_object_field(
                    "javaVersion",
                    &self.r#java_version,
                ),
                to_pulumi_object_field(
                    "nodeVersion",
                    &self.r#node_version,
                ),
                to_pulumi_object_field(
                    "phpVersion",
                    &self.r#php_version,
                ),
                to_pulumi_object_field(
                    "python",
                    &self.r#python,
                ),
                to_pulumi_object_field(
                    "tomcatVersion",
                    &self.r#tomcat_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WindowsWebAppSlotSiteConfigApplicationStack {
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
                        let field_value = match fields_map.get("currentStack") {
                            Some(value) => value,
                            None => bail!("Missing field 'currentStack' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_image_name: {
                        let field_value = match fields_map.get("dockerImageName") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerImageName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_password: {
                        let field_value = match fields_map.get("dockerRegistryPassword") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerRegistryPassword' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_url: {
                        let field_value = match fields_map.get("dockerRegistryUrl") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerRegistryUrl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#docker_registry_username: {
                        let field_value = match fields_map.get("dockerRegistryUsername") {
                            Some(value) => value,
                            None => bail!("Missing field 'dockerRegistryUsername' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_core_version: {
                        let field_value = match fields_map.get("dotnetCoreVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetCoreVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dotnet_version: {
                        let field_value = match fields_map.get("dotnetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#java_embedded_server_enabled: {
                        let field_value = match fields_map.get("javaEmbeddedServerEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaEmbeddedServerEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#node_version: {
                        let field_value = match fields_map.get("nodeVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'nodeVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#python: {
                        let field_value = match fields_map.get("python") {
                            Some(value) => value,
                            None => bail!("Missing field 'python' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tomcat_version: {
                        let field_value = match fields_map.get("tomcatVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'tomcatVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
