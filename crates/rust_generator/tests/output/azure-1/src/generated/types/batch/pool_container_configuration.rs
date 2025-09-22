#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolContainerConfiguration {
    /// A list of container image names to use, as would be specified by `docker pull`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerImageNames")]
    pub r#container_image_names: Option<Vec<String>>,
    /// One or more `container_registries` blocks as defined below. Additional container registries from which container images can be pulled by the pool's VMs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "containerRegistries")]
    pub r#container_registries: Option<Vec<super::super::types::batch::PoolContainerConfigurationContainerRegistry>>,
    /// The type of container configuration. Possible value is `DockerCompatible`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
