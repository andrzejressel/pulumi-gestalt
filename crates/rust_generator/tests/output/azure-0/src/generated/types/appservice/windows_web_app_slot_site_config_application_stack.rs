#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfigApplicationStack {
    /// The Application Stack for the Windows Web App. Possible values include `dotnet`, `dotnetcore`, `node`, `python`, `php`, and `java`.
    /// 
    /// > **NOTE:** Whilst this property is Optional omitting it can cause unexpected behaviour, in particular for display of settings in the Azure Portal.
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
    /// The version of Java to use when `current_stack` is set to `java`. Possible values include `1.7`, `1.8`, `11` and `17`. Required with `java_container` and `java_container_version`.
    /// 
    /// > **NOTE:** For compatible combinations of `java_version`, `java_container` and `java_container_version` users can use `az webapp list-runtimes` from command line.
    #[builder(into)]
    #[serde(rename = "javaVersion")]
    pub r#java_version: Option<String>,
    /// The version of node to use when `current_stack` is set to `node`. Possible values include `~12`, `~14`, `~16`, `~18` and `~20`.
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
    /// The app is a Python app. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "python")]
    pub r#python: Option<bool>,
    /// The version of Tomcat the Java App should use.
    /// 
    /// > **NOTE:** See the official documentation for current supported versions.
    #[builder(into)]
    #[serde(rename = "tomcatVersion")]
    pub r#tomcat_version: Option<String>,
}
