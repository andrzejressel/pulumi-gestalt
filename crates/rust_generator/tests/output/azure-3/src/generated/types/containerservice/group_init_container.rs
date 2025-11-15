#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupInitContainer {
    /// A list of commands which should be run on the container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "commands")]
    pub r#commands: Option<Vec<String>>,
    /// A list of environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Option<std::collections::HashMap<String, String>>,
    /// The container image name. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: String,
    /// Specifies the name of the Container. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of sensitive environment variables to be set on the container. Specified as a map of name/value pairs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secureEnvironmentVariables")]
    pub r#secure_environment_variables: Option<std::collections::HashMap<String, String>>,
    /// The definition of the security context for this container as documented in the `security` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "securities")]
    pub r#securities: Option<Vec<super::super::types::containerservice::GroupInitContainerSecurity>>,
    /// The definition of a volume mount for this container as documented in the `volume` block below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::containerservice::GroupInitContainerVolume>>,
}
