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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
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
                    "dotnetVersion",
                    &self.r#dotnet_version,
                ),
                to_pulumi_object_field(
                    "goVersion",
                    &self.r#go_version,
                ),
                to_pulumi_object_field(
                    "javaServer",
                    &self.r#java_server,
                ),
                to_pulumi_object_field(
                    "javaServerVersion",
                    &self.r#java_server_version,
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
                    "pythonVersion",
                    &self.r#python_version,
                ),
                to_pulumi_object_field(
                    "rubyVersion",
                    &self.r#ruby_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                    r#dotnet_version: {
                        let field_value = match fields_map.get("dotnetVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'dotnetVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#go_version: {
                        let field_value = match fields_map.get("goVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'goVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_server: {
                        let field_value = match fields_map.get("javaServer") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaServer' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#java_server_version: {
                        let field_value = match fields_map.get("javaServerVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'javaServerVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#python_version: {
                        let field_value = match fields_map.get("pythonVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'pythonVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ruby_version: {
                        let field_value = match fields_map.get("rubyVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'rubyVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
