#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SourceControlGithubActionConfigurationContainerConfiguration {
    /// The image name for the build. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "imageName")]
    pub r#image_name: String,
    /// The password used to upload the image to the container registry. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryPassword")]
    pub r#registry_password: Option<String>,
    /// The server URL for the container registry where the build will be hosted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryUrl")]
    pub r#registry_url: String,
    /// The username used to upload the image to the container registry. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "registryUsername")]
    pub r#registry_username: Option<String>,
}
