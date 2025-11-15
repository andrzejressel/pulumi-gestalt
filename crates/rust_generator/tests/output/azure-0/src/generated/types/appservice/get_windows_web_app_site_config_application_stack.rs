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
