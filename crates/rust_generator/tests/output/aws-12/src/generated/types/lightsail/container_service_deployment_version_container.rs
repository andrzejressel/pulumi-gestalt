#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ContainerServiceDeploymentVersionContainer {
    /// The launch command for the container. A list of string.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// The name for the container.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: String,
    /// A key-value map of the environment variables of the container.
    #[builder(into)]
    #[serde(rename = "environment")]
    pub r#environment: Option<std::collections::HashMap<String, String>>,
    /// The name of the image used for the container. Container images sourced from your Lightsail container service, that are registered and stored on your service, start with a colon (`:`). For example, `:container-service-1.mystaticwebsite.1`. Container images sourced from a public registry like Docker Hub don't start with a colon. For example, `nginx:latest` or `nginx`.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// A key-value map of the open firewall ports of the container. Valid values: `HTTP`, `HTTPS`, `TCP`, `UDP`.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<std::collections::HashMap<String, String>>,
}
