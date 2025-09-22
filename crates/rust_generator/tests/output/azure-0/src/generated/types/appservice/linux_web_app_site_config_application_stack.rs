#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
