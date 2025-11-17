#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderRegistryAuth {
    /// Address of the registry
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: String,
    #[builder(into)]
    #[serde(rename = "authDisabled")]
    pub r#auth_disabled: Option<bool>,
    /// Path to docker json file for registry auth. Defaults to `~/.docker/config.json`. If `DOCKER_CONFIG` is set, the value of `DOCKER_CONFIG` is used as the path. `config_file` has predencen over all other options.
    #[builder(into)]
    #[serde(rename = "configFile")]
    pub r#config_file: Option<String>,
    /// Plain content of the docker json file for registry auth. `config_file_content` has precedence over username/password.
    #[builder(into)]
    #[serde(rename = "configFileContent")]
    pub r#config_file_content: Option<String>,
    /// Password for the registry. Defaults to `DOCKER_REGISTRY_PASS` env variable if set.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Username for the registry. Defaults to `DOCKER_REGISTRY_USER` env variable if set.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
