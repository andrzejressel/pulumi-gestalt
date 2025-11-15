#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskDockerStep {
    /// Specifies a map of arguments to be used when executing this step.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: Option<std::collections::HashMap<String, String>>,
    /// Should the image cache be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "cacheEnabled")]
    pub r#cache_enabled: Option<bool>,
    /// The token (Git PAT or SAS token of storage account blob) associated with the context for this step.
    #[builder(into)]
    #[serde(rename = "contextAccessToken")]
    pub r#context_access_token: String,
    /// The URL (absolute or relative) of the source context for this step. If the context is an url you can reference a specific branch or folder via `#branch:folder`.
    #[builder(into)]
    #[serde(rename = "contextPath")]
    pub r#context_path: String,
    /// The Dockerfile path relative to the source context.
    #[builder(into)]
    #[serde(rename = "dockerfilePath")]
    pub r#dockerfile_path: String,
    /// Specifies a list of fully qualified image names including the repository and tag.
    #[builder(into)]
    #[serde(rename = "imageNames")]
    pub r#image_names: Option<Vec<String>>,
    /// Should the image built be pushed to the registry or not? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "pushEnabled")]
    pub r#push_enabled: Option<bool>,
    /// Specifies a map of *secret* arguments to be used when executing this step.
    #[builder(into)]
    #[serde(rename = "secretArguments")]
    pub r#secret_arguments: Option<std::collections::HashMap<String, String>>,
    /// The name of the target build stage for the docker build.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<String>,
}
