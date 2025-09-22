#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateContainerEnv {
    /// The name of the environment variable.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Name of the Container App secret from which to pull the environment variable value.
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Option<String>,
    /// The value of the environment variable.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
